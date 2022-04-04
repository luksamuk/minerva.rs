// controller/estoque.rs -- Uma parte de Minerva.rs
// Copyright (C) 2021 Lucas S. Vieira
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Ferramentas para tráfego de dados entre as rotas de estoque/movimentação de
//! estoque e o banco de dados.
//!
//! As ferramentas deste módulo realizam o tráfego de dados entre as respectivas
//! rotas de posição e movimentação de estoque e as tabelas relacionadas a estas
//! operações.

use super::log::*;
use crate::model::estoque::*;
use crate::routes::respostas::Resposta;
use comfy_table::Table;
use diesel::prelude::*;
use serde_json::json;

/// Realiza início de estoque.
///
/// Esta função realiza um início de estoque, caso já não tenha sido feito. A
/// função realizará verificações para avaliar se o produto está cadastrado no
/// sistema, se o estoque já não foi iniciado, e se os dados iniciais recebidos
/// são válidos.
///
/// Caso o produto não exista, será retornado um erro 404. Do contrário, caso a
/// posição inicial de estoque possua um erro em sua validação, será retornado
/// um erro 412, dada a invalidade semântica dos dados.
pub fn inicia_estoque(conexao: &PgConnection, recv: Estoque) -> Resposta {
    use super::produtos;
    use crate::model::schema::estoque;
    use bigdecimal::{Signed, Zero};

    // 1. Verifica se o produto existe.
    if produtos::get_produto(conexao, recv.produto_id).is_none() {
        return Resposta::NaoEncontrado(
            json!({
                "mensagem": "Produto não encontrado."
            })
            .to_string(),
        );
    }

    // 2. Verifica se já não houve início de estoque.
    if get_estoque(conexao, recv.produto_id).is_some() {
        return Resposta::ErroSemantico(
            json!({
                "mensagem":
                    format!(
                        "Já foi realizado início de estoque para o produto {}.",
                        recv.produto_id
                    )
            })
            .to_string(),
        );
    }

    // 3. Verifica se quantidade e preço unitário são válidos.
    if recv.precounitario.is_zero() || recv.precounitario.is_negative() {
        return Resposta::ErroSemantico(
            json!({
                "mensagem": "O preço unitário deve ser maior que zero."
            })
            .to_string(),
        );
    }

    if recv.quantidade.is_negative() {
        return Resposta::ErroSemantico(
            json!({
                "mensagem": "A quantidade não pode ser negativa."
            })
            .to_string(),
        );
    }

    // 4. Realiza início de estoque.
    match diesel::insert_into(estoque::table)
        .values(&recv)
        .get_result::<Estoque>(conexao)
    {
        Ok(est) => {
            let _ = registra_log(
                conexao,
                String::from("ESTOQUE"),
                String::from("TO-DO"),
                DBOperacao::Insercao,
                Some(format!("Início de estoque do produto {}", est.produto_id)),
            );
            Resposta::Ok(serde_json::to_string(&est).unwrap())
        }
        Err(e) => {
            if let diesel::result::Error::DatabaseError(_, _) = &e {
                Resposta::ErroSemantico(
                    json!({
                        "mensagem": e.to_string()
                    })
                    .to_string(),
                )
            } else {
                Resposta::ErroSemantico(
                    json!({
                        "mensagem":
                        "Erro intero ao realizar início de estoque. \
                         Contate o suporte para mais informações."
                    })
                    .to_string(),
                )
            }
        }
    }
}

/// Realiza uma movimentação de estoque de um produto.
///
/// Esta função realiza uma movimentação de estoque do referido produto. A
/// função também efetua validações para garantir que o produto exista, e que
/// sua posição de estoque também exista, do contrário, será retornado um erro
/// 404.
///
/// Além disso, a função verificará se o preço unitário foi informado como
/// negativo ou zero, se o preço do frete, caso informado, tenha sido informado
/// como negativo, e se a movimentação a ser registrada colocará o estoque como
/// negativo. Qualquer uma dessas situações classifica-se como erro semântico,
/// retornando um erro 412.
pub fn movimenta_estoque(conexao: &PgConnection, recv: MovEstoqueRecv) -> Resposta {
    use super::produtos;
    use bigdecimal::{Signed, Zero};

    // 1. Verifica se o produto existe.
    if produtos::get_produto(conexao, recv.produto_id).is_none() {
        return Resposta::NaoEncontrado(
            json!({
                "mensagem": "Produto não encontrado"
            })
            .to_string(),
        );
    }

    // 2. Verifica se foi feito início de estoque.
    let estoque_atual = get_estoque(conexao, recv.produto_id);
    if estoque_atual.is_none() {
        return Resposta::NaoEncontrado(
            json!({
                "mensagem":
                    format!(
                        "Necessário efetuar início de estoque para o produto {}",
                        recv.produto_id
                    )
            })
            .to_string(),
        );
    }

    // 3.1. Verifica se o preço unitário é negativo ou se foi zerado.
    if recv.preco_unitario.is_negative() || recv.preco_unitario.is_zero() {
        return Resposta::ErroSemantico(
            json!({
                "mensagem": "Preço unitário deve ser maior que zero"
            })
            .to_string(),
        );
    }

    // 3.2. Verifica se o preço do frete, quando informado, é negativo.
    if recv.preco_frete.is_some() {
        let frete = recv.preco_frete.clone().unwrap();
        if frete.is_negative() {
            return Resposta::ErroSemantico(
                json!({
                "mensagem": "Preço do frete não pode ser negativo"
                })
                .to_string(),
            );
        }
    }

    // 3.3. Verifica se a movimentação vai colocar o estoque como negativo
    //      ou zerar o preço.
    let estoque_atual = estoque_atual.unwrap();
    let nova_qtd_estoque = estoque_atual.quantidade.clone() + recv.quantidade.clone();
    if nova_qtd_estoque.is_negative() {
        return Resposta::ErroSemantico(
            json!({
                "mensagem":
                    format!(
                        "Movimentações de estoque não podem torná-lo \
		 negativo! Estoque atual: {}",
                        estoque_atual.quantidade
                    )
            })
            .to_string(),
        );
    }

    // 4.1. Cadastra o movimento.
    let novo_movimento = NovoMovEstoque::from(recv);
    let movimento = {
        use crate::model::schema::mov_estoque;
        match diesel::insert_into(mov_estoque::table)
            .values(&novo_movimento)
            .get_result::<MovEstoque>(conexao)
        {
            Ok(movimen) => {
                let _ = registra_log(
                    conexao,
                    String::from("MOV_ESTOQUE"),
                    String::from("TO-DO"),
                    DBOperacao::Insercao,
                    Some(format!("Movimento de estoque {}", movimen.id)),
                );
                movimen
            }
            Err(e) => {
                if let diesel::result::Error::DatabaseError(_, _) = &e {
                    return Resposta::ErroSemantico(
                        json!({
                        "mensagem": e.to_string()
                        })
                        .to_string(),
                    );
                } else {
                    return Resposta::ErroSemantico(
                        json!({
                        "mensagem":
                                    "Erro interno ao realizar movimentação de estoque. \
                                     Contate o suporte para mais informações."
                        })
                        .to_string(),
                    );
                }
            }
        }
    };

    // 4.2. Modifica o estoque.
    let mod_estoque = {
        use crate::model::schema::estoque::dsl::*;
        diesel::update(estoque.filter(produto_id.eq(&novo_movimento.produto_id)))
            .set((
                quantidade.eq(&nova_qtd_estoque),
                precounitario.eq(&novo_movimento.preco_unitario),
            ))
            .get_result::<Estoque>(conexao)
    };

    match mod_estoque {
        Ok(est) => {
            let _ = registra_log(
                conexao,
                String::from("ESTOQUE"),
                String::from("TO-DO"),
                DBOperacao::Alteracao,
                Some(format!("Altera estoque do produto {}", est.produto_id)),
            );

            // 5.1. Retorna o movimento
            Resposta::Ok(serde_json::to_string(&movimento).unwrap())
        }
        Err(e) => {
            // 5.2. Em caso de erro, realiza rollback da movimentação
            {
                use crate::model::schema::mov_estoque::dsl::*;
                diesel::delete(mov_estoque.filter(id.eq(&movimento.id)))
                    .execute(conexao)
                    .expect("Erro no rollback de movimentação de estoque!");
                let _ = registra_log(
                    conexao,
                    String::from("MOV_ESTOQUE"),
                    String::from("TO-DO"),
                    DBOperacao::Remocao,
                    Some(format!("Rollback de movimento de estoque {}", movimento.id)),
                );
            }

            if let diesel::result::Error::DatabaseError(_, _) = &e {
                Resposta::ErroSemantico(
                    json!({
                        "mensagem": e.to_string()
                    })
                    .to_string(),
                )
            } else {
                Resposta::ErroSemantico(
                    json!({
                        "mensagem":
                                "Erro interno ao atualizar estoque. \
                                 Contate o suporte para mais informações."
                    })
                    .to_string(),
                )
            }
        }
    }
}

/// Retorna a posição de estoque de um produto.
///
/// Esta função retorna um Option que poderá conter a posição de estoque de um
/// produto com o id informado. Esta função verifica apenas se houve início de
/// estoque do produto, mas não verifica se o produto existe.
pub fn get_estoque(conexao: &PgConnection, prod_id: i32) -> Option<Estoque> {
    use crate::model::schema::estoque::dsl::*;
    let estoque_req = estoque
        .filter(produto_id.eq(&prod_id))
        .load::<Estoque>(conexao)
        .expect("Erro ao carregar estoque");
    estoque_req.first().cloned()
}

/// Une as informações de uma posição de estoque de um produto com os dados do
/// produto referenciado.
fn transforma_estoque_retorno(conexao: &PgConnection, e: &Estoque) -> EstoqueRepr {
    use super::produtos;
    let p = produtos::get_produto(conexao, e.produto_id).unwrap();
    EstoqueRepr {
        id: p.id,
        descricao: p.descricao.clone(),
        unidsaida: p.unidsaida,
        quantidade: e.quantidade.clone(),
        preco_unitario: e.precounitario.clone(),
    }
}

/// Lista uma quantidade limitada de posições de estoque com dados de produto.
///
/// Retorna um Vec com estruturas que representam a união entre dados de um
/// produto e de sua posição de estoque. A quantidade de estruturas retornadas
/// não será superior a `limite`.
pub fn lista_estoque(conexao: &PgConnection, limite: i64) -> Vec<EstoqueRepr> {
    use crate::model::schema::estoque;
    estoque::table
        .limit(limite)
        .load::<Estoque>(conexao)
        .expect("Erro ao carregar estoque")
        .iter()
        .map(|e| transforma_estoque_retorno(conexao, e))
        .collect()
}

/// Mostra a posição de estoque de um produto com seus respectivos dados.
///
/// Retorna um Option que poderá conter os dados de posição de estoque de um
/// produto, junto com seus dados de cadastro. Os dados só serão retornados se
/// o sistema encontrar a posição de estoque do produto e seus dados
/// correspondentes, respectivamente.
pub fn mostra_estoque(conexao: &PgConnection, prod_id: i32) -> Option<EstoqueRepr> {
    get_estoque(conexao, prod_id)
        .as_ref()
        .map(|e| transforma_estoque_retorno(conexao, e))
}

/// Lista uma quantidade limitada de posições de estoque com dados de produto,
/// em uma tabela escrita como texto-plano.
///
/// O retorno será uma string em texto-plano, formatada para assemelhar-se a uma
/// tabela. A tabela não terá mais linhas do que a quantidade descrita pelo
/// parâmetro `limite`.
pub fn lista_movimentos_texto(conexao: &PgConnection, limite: i64) -> String {
    let movimentos = recupera_movimentos(conexao, limite);
    let mut table = Table::new();
    prepara_tabela(&mut table);
    processa_tabela(conexao, &movimentos, &mut table);
    format!("{}\n", table)
}

/// Prepara o cabeçalho da tabela de movimentações de estoque.
fn prepara_tabela(table: &mut Table) {
    use comfy_table::presets::ASCII_BORDERS_ONLY_CONDENSED;
    table
        .load_preset(ASCII_BORDERS_ONLY_CONDENSED)
        .set_header(vec![
            "Produto",
            "Documento",
            "Tipo",
            "Quantidade",
            "Preço Unit.",
            "Frete",
            "Data/Hora",
        ]);
}

/// Adiciona dados de um vetor de movimentações de estoque a uma estrutura de
/// tabela em texto-plano.
fn processa_tabela(conexao: &PgConnection, movimentos: &[MovEstoque], table: &mut Table) {
    use super::produtos;
    use bigdecimal::Signed;
    for mov in movimentos {
        let nome_produto = match produtos::get_produto(conexao, mov.produto_id) {
            Some(p) => format!("{} - {:.20}", p.id, p.descricao),
            None => format!("Produto {}", mov.produto_id),
        };

        let tipo_movimento = String::from(if mov.quantidade.is_positive() {
            "Entrada"
        } else {
            "Saída"
        });

        table.add_row(vec![
            nome_produto,
            mov.docto.clone(),
            tipo_movimento,
            mov.quantidade.abs().to_string(),
            mov.preco_unitario.to_string(),
            mov.preco_frete.to_string(),
            mov.datahora.to_string(),
        ]);
    }
}

/// Recupera movimentações de estoque a partir do banco de dados, em ordem
/// decrescente de data. A quantidade de movimentos retornada não será superior
/// à quantidade informada através de `limite`.
pub fn recupera_movimentos(conexao: &PgConnection, limite: i64) -> Vec<MovEstoque> {
    use crate::model::schema::mov_estoque::dsl::*;
    mov_estoque
        .order(datahora.desc())
        .limit(limite)
        .load::<MovEstoque>(conexao)
        .expect("Erro ao recuperar movimentações de estoque")
}

/// Recupera movimentações de estoque a partir do banco de dados, em ordem
/// decrescente de data, impressas em texto-plano formatado para parecer uma
/// tabela.
///
/// As movimentações recuperadas deverão obedecer a um filtro `is_entrada`, que
/// determinará se serão retornadas como movimentações de entrada ou saída de
/// produtos. A quantidade de movimentos retornada não será superior à
/// quantidade informada através de `limite`.
pub fn lista_movimentos_texto_filtrado(
    conexao: &PgConnection,
    limite: i64,
    is_entrada: bool,
) -> String {
    let movimentos = recupera_movimentos_filtrado(conexao, limite, is_entrada);
    let mut table = Table::new();
    prepara_tabela(&mut table);
    processa_tabela(conexao, &movimentos, &mut table);
    format!("{}\n", table)
}

/// Recupera movimentações de estoque a partir do banco de dados, em ordem
/// decrescente de data.
///
/// As movimentações recuperadas deverão obedecer a um filtro `is_entrada`, que
/// determinará se serão retornadas como movimentações de entrada ou saída de
/// produtos. A quantidade de movimentos retornada não será superior à
/// quantidade informada através de `limite`.
pub fn recupera_movimentos_filtrado(
    conexao: &PgConnection,
    limite: i64,
    is_entrada: bool,
) -> Vec<MovEstoque> {
    use crate::model::schema::mov_estoque::dsl::*;
    use bigdecimal::{BigDecimal, Zero};
    let z: BigDecimal = Zero::zero();

    let query = mov_estoque.order(datahora.desc()).limit(limite);

    if is_entrada {
        query
            .filter(quantidade.ge(z))
            .load::<MovEstoque>(conexao)
            .expect("Erro ao recuperar movimentações de estoque")
    } else {
        query
            .filter(quantidade.lt(z))
            .load::<MovEstoque>(conexao)
            .expect("Erro ao recuperar movimentações de estoque")
    }
}
