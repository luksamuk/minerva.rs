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
use crate::model::produto::{ Produto, ProdutoRepr, ProdutoRecv };
use crate::model::schema::produto::dsl::*;

pub fn lista_produtos(conexao: &PgConnection, limite: i64) -> Vec<ProdutoRepr> {
    produto::table.limit(limite)
        .load::<Produto>(conexao)
        .expect("Erro ao carregar produtos")
        .iter().map(|p| p.into_repr()).collect()
}

pub fn get_produto(conexao: &PgConnection, prodid: i32) -> Option<ProdutoRepr> {    
    let prod_req = produto.filter(id.eq(&prodid))
        .load::<Produto>(conexao)
        .expect("Erro ao carregar produto");
    match prod_req.first() {
        None => None,
        Some(p) => Some(p.into_repr()),
    }
}

pub fn deleta_produto(conexao: &PgConnection, prodid: i32) {
    diesel::delete(produto.filter(id.eq(&prodid)))
        .execute(conexao)
        .expect("Erro ao deletar produto");
}

pub fn registra_produto(conexao: &PgConnection, dados: ProdutoRecv) -> i32 {
    let dados = dados.into_new();
    let p: Produto =
        diesel::insert_into(produto::table)
        .values(&dados)
        .get_result(conexao)
        .expect("Erro ao cadastrar produto");
    p.id
}
