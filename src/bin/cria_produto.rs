// bin/cria_produto.rs -- Uma parte de Minerva.rs
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

extern crate minerva;
extern crate diesel;

use minerva::model::schema::produto;
use minerva::model::produto::{ NovoProduto, Produto };
use diesel::prelude::*;

use minerva::inpututils::*;

fn main() {
    let pool = minerva::db::create_connection_pool();
    let conn = pool.get().unwrap();
    let mut novoproduto = NovoProduto::new();

    prompt("Descricao: ");
    novoproduto.descricao = get_input();
    prompt("Unid. saída: ");
    novoproduto.unidsaida = get_input().to_uppercase();
    prompt("Qtd. estoque: ");
    novoproduto.qtdestoque = get_numeric();
    prompt("Preço de venda: ");
    novoproduto.precovenda = get_numeric();

    let _p: Produto =
        diesel::insert_into(produto::table)
        .values(&novoproduto)
        .get_result(&conn)
        .expect("Erro ao salvar produto");
}
