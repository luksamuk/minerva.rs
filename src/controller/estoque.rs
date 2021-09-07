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

use diesel::prelude::*;
use crate::model::estoque::*;
use super::log::*;
use crate::routes::respostas::Resposta;

pub fn inicia_estoque(conexao: &PgConnection, recv: Estoque) -> Resposta {
    use super::produtos;
    use crate::model::schema::estoque;
    use bigdecimal::{ Zero, Signed };
    
    // 1. Verifica se o produto existe.
    if produtos::get_produto(conexao, recv.produto_id).is_none() {
        return Resposta::NaoEncontrado(String::from(
            "{ \"mensagem\": \"Produto não encontrado\" }"));
    }
    
    // 2. Verifica se já não houve início de estoque.
    if get_estoque(conexao, recv.produto_id).is_some() {
        return Resposta::ErroSemantico(format!(
            "{{ \"mensagem\": \
             \"Já foi realizado início de estoque para o produto {}\" }}",
            recv.produto_id));
    }

    // 3. Verifica se quantidade e preço unitário são válidos.
    if recv.precounitario.is_zero() || recv.precounitario.is_negative() {
        return Resposta::ErroSemantico(String::from(
            "{ \"mensagem\": \"O preço unitário deve ser maior que zero.\" }"));
    }

    if recv.quantidade.is_negative() {
        return Resposta::ErroSemantico(String::from(
            "{ \"mensagem\": \"A quantidade não pode ser negativa.\" }"));
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
                Some(format!("Início de estoque do produto {}",
                             est.produto_id)));
            Resposta::Ok(serde_json::to_string(&est).unwrap())
        },
        Err(e) => {
            if let diesel::result::Error::DatabaseError(_, _) = &e {
                return Resposta::ErroSemantico(format!(
                    "{{ \"mensagem\": \"{}\" }}", e));
            } else {
                return Resposta::ErroSemantico(String::from(
                    "Erro interno ao realizar início de estoque. \
                     Contate o suporte para mais informações."));
            }
        },
    }
    
}

pub fn movimenta_estoque(conexao: &PgConnection, recv: MovEstoqueRecv) -> Resposta {
    use super::produtos;
    use bigdecimal::{ Zero, Signed };

    // 1. Verifica se o produto existe.
    if produtos::get_produto(conexao, recv.produto_id).is_none() {
        return Resposta::NaoEncontrado(String::from(
            "{ \"mensagem\": \"Produto não encontrado\" }"));
    }

    // 2. Verifica se foi feito início de estoque.
    let estoque_atual = get_estoque(conexao, recv.produto_id);
    if estoque_atual.is_none() {
        return Resposta::ErroSemantico(format!(
            "{{ \"mensagem\": \
             \"Necessário efetuar início de estoque para o produto {}\" \
             }}", recv.produto_id));
    }
    
    // 3.1. Verifica se o preço unitário é negativo ou se foi zerado.
    if recv.preco_unitario.is_negative() || recv.preco_unitario.is_zero() {
        return Resposta::ErroSemantico(String::from(
            "{ \"mensagem\": \"Preço unitário deve ser maior que zero\" }"));
    }

    // 3.2. Verifica se o preço do frete, quando informado, é negativo.
    if recv.preco_frete.is_some() {
        let frete = recv.preco_frete.clone().unwrap();
        if frete.is_negative() {
            return Resposta::ErroSemantico(String::from(
                "{ \"mensagem\": \"Preço de frete não pode ser negativo\" }"));
        }
    }

    // 3.3. Verifica se a movimentação vai colocar o estoque como negativo
    //      ou zerar o preço.
    let estoque_atual = estoque_atual.unwrap();
    let nova_qtd_estoque =
        estoque_atual.quantidade.clone() + recv.quantidade.clone();
    if nova_qtd_estoque.is_negative() {
        return Resposta::ErroSemantico(format!(
            "{{ \"mensagem\": \"Movimentações de estoque não podem torná-lo \
             negativo! Estoque atual: {}\" }}", estoque_atual.quantidade));
    }
    
    // 4.1. Cadastra o movimento.
    let novo_movimento = NovoMovEstoque::from(recv);
    let movimento = {
        use crate::model::schema::mov_estoque::dsl::*;
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
                    Some(format!("Movimento de estoque {}",
                                 movimen.id)));
                movimen.clone()
            },
            Err(e) => {
                if let diesel::result::Error::DatabaseError(_, _) = &e {
                    return Resposta::ErroSemantico(format!(
                        "{{ \"mensagem\": \"{}\" }}", e));
                } else {
                    return Resposta::ErroSemantico(String::from(
                        "Erro interno ao realizar movimentação de estoque. \
                         Contate o suporte para mais informações."));
                }
            },
        }
    };

    // 4.2. Modifica o estoque.
    let mod_estoque = {
        use crate::model::schema::estoque::dsl::*;
        diesel::update(estoque.filter(
            produto_id.eq(&novo_movimento.produto_id)))
            .set((
                quantidade.eq(&nova_qtd_estoque),
                precounitario.eq(&novo_movimento.preco_unitario)
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
                Some(format!("Altera estoque do produto {}",
                             est.produto_id)));
            
            // 5.1. Retorna o movimento
            Resposta::Ok(serde_json::to_string(&movimento).unwrap())
        },
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
                    Some(format!("Rollback de movimento de estoque {}",
                                 movimento.id)));
            }
            
            if let diesel::result::Error::DatabaseError(_, _) = &e {
                return Resposta::ErroSemantico(format!(
                    "{{ \"mensagem\": \"{}\" }}", e));
            } else {
                return Resposta::ErroSemantico(String::from(
                    "Erro interno ao atualizar estoque. \
                     Contate o suporte para mais informações."));
            }
        }
    }
}

pub fn get_estoque(conexao: &PgConnection, prod_id: i32) -> Option<Estoque> {
    use crate::model::schema::estoque::dsl::*;
    let estoque_req = estoque.filter(produto_id.eq(&prod_id))
        .load::<Estoque>(conexao)
        .expect("Erro ao carregar estoque");
    match estoque_req.first() {
        None => None,
        Some(e_ref) => Some(e_ref.clone()),
    }
}

