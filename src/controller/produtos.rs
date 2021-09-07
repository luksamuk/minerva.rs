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

use diesel::prelude::*;
use crate::model::schema::produto;
use crate::model::produto::{ Produto, NovoProduto };
use crate::model::schema::produto::dsl::*;
use bigdecimal::BigDecimal;
use crate::routes::respostas::Resposta;
use super::log::*;

pub fn lista_produtos(conexao: &PgConnection, limite: i64) -> Vec<Produto> {
    produto::table.limit(limite)
        .load::<Produto>(conexao)
        .expect("Erro ao carregar produtos")
}

pub fn get_produto(conexao: &PgConnection, prod_id: i32) -> Option<Produto> {
    let prod_req = produto.filter(id.eq(&prod_id))
        .load::<Produto>(conexao)
        .expect("Erro ao carregar produto");
    match prod_req.first() {
        None => None,
        Some(p_ref) => Some(p_ref.clone()),
    }
}

pub fn deleta_produto(conexao: &PgConnection, prodid: i32) {
    diesel::delete(produto.filter(id.eq(&prodid)))
        .execute(conexao)
        .expect("Erro ao deletar produto");
    let _ = registra_log(
        conexao,
        String::from("PRODUTO"),
        String::from("TO-DO"),
        DBOperacao::Remocao,
        Some(format!("Produto {}", prodid)));
}

pub fn deleta_todos(conexao: &PgConnection) -> usize {
    let num_deletados = diesel::delete(produto::table)
        .execute(conexao)
        .expect("Erro ao deletar produtos");
    let _ = registra_log(
        conexao,
        String::from("PRODUTO"),
        String::from("TO-DO"),
        DBOperacao::Remocao,
        Some(String::from("Removendo todos os produtos")));
    num_deletados
}

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
                Some(format!("Produto {}", prod.id)));
            Ok(prod.id)
        },
        Err(e) => {
            if let diesel::result::Error::DatabaseError(_, _) = &e {
                Err(format!("{}", e))
            } else {
                Err(String::from("Erro interno ao cadastrar produto. \
                                  Contate o suporte para mais informações."))
            }
        },
    }
}

