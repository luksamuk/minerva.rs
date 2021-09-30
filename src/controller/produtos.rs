// controller/produtos.rs -- Uma parte de Minerva.rs
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

//! Ferramentas para tráfego de dados entre as rotas de produtos e o banco de
//! dados.
//! 
//! As ferramentas deste módulo realizam o tráfego entre os dados recebidos
//! através das rotas para gerenciamento de produto e a tabela `produto` do
//! banco de dados e relacionadas.

use super::log::*;
use crate::model::produto::{NovoProduto, Produto};
use crate::model::schema::produto;
use crate::model::schema::produto::dsl::*;
use diesel::prelude::*;

/// Lista uma quantidade limitada de produtos cadastrados no sistema.
/// 
/// Retorna um Vec com estruturas que representam os dados de um produto. A
/// quantidade de produtos retornada não deverá exceder a informada no
/// parâmetro `limite`.
pub fn lista_produtos(conexao: &PgConnection, limite: i64) -> Vec<Produto> {
    produto::table
        .limit(limite)
        .load::<Produto>(conexao)
        .expect("Erro ao carregar produtos")
}

/// Mostra os dados de um único produto, caso existente.
/// 
/// Retorna um Option que poderá conter os dados de um produto. Caso o produto
/// com o id informado exista, uma estrutura que representa seus dados será
/// retornada.
pub fn get_produto(conexao: &PgConnection, prod_id: i32) -> Option<Produto> {
    let prod_req = produto
        .filter(id.eq(&prod_id))
        .load::<Produto>(conexao)
        .expect("Erro ao carregar produto");
    prod_req.first().cloned()
}

/// Deleta um produto em específico do banco de dados.
/// 
/// O produto a ser deletado deverá ter seu id informado através do parâmetro
/// `prodid`. Esta função assume que o produto de id informada exista no banco
/// de dados.
pub fn deleta_produto(conexao: &PgConnection, prodid: i32) {
    diesel::delete(produto.filter(id.eq(&prodid)))
        .execute(conexao)
        .expect("Erro ao deletar produto");
    let _ = registra_log(
        conexao,
        String::from("PRODUTO"),
        String::from("TO-DO"),
        DBOperacao::Remocao,
        Some(format!("Produto {}", prodid)),
    );
}

/// Deleta todos os produtos cadastrados no banco de dados.
/// 
/// Será retornada a quantidade de registros removidos no processo. Utilize
/// esta função com cuidado.
pub fn deleta_todos(conexao: &PgConnection) -> usize {
    let num_deletados = diesel::delete(produto::table)
        .execute(conexao)
        .expect("Erro ao deletar produtos");
    let _ = registra_log(
        conexao,
        String::from("PRODUTO"),
        String::from("TO-DO"),
        DBOperacao::Remocao,
        Some(String::from("Removendo todos os produtos")),
    );
    num_deletados
}

/// Registra um novo produto no banco de dados.
/// 
/// Esta função assume que os dados de registro de novo produto sejam válidos.
/// Caso o produto seja cadastrado, será retornado seu id no banco de dados.
/// Caso contrário, será retornada uma mensagem de erro em String.
pub fn registra_produto(conexao: &PgConnection, mut dados: NovoProduto) -> Result<i32, String> {
    dados.unidsaida = dados.unidsaida.to_uppercase();
    match diesel::insert_into(produto::table)
        .values(&dados)
        .get_result::<Produto>(conexao)
    {
        Ok(prod) => {
            let _ = registra_log(
                conexao,
                String::from("PRODUTO"),
                String::from("TO-DO"),
                DBOperacao::Insercao,
                Some(format!("Produto {}", prod.id)),
            );
            Ok(prod.id)
        }
        Err(e) => {
            if let diesel::result::Error::DatabaseError(_, _) = &e {
                Err(format!("{}", e))
            } else {
                Err(String::from(
                    "Erro interno ao cadastrar produto. \
                                  Contate o suporte para mais informações.",
                ))
            }
        }
    }
}
