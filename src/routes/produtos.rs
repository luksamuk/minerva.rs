// routes/produtos.rs -- Uma parte de Minerva.rs
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

use rocket::Route;
use rocket::serde::json::Json;
use crate::controller::produtos;
use rocket::State;
use crate::db::ConexaoPool;
use diesel::prelude::*;
use crate::model::produto::ProdutoRecv;
use super::respostas::Resposta;
use crate::model::estoque::MovEstoque;

pub fn constroi_rotas() -> Vec<Route> {
    routes![
        index,
        retorna_produto,
        deleta_todos,
        deleta,
        cadastra,
    ]
}

#[get("/")]
fn index(pool: &State<ConexaoPool>) -> Resposta {
    let conexao = pool.get().unwrap();
    let vec_produtos = produtos::lista_produtos(&conexao, 100);
    Resposta::Ok(serde_json::to_string(&vec_produtos).unwrap())
}

#[get("/<prod_id>")]
fn retorna_produto(pool: &State<ConexaoPool>, prod_id: i32) -> Resposta {
    let conexao = pool.get().unwrap();
    match produtos::get_produto(&conexao, prod_id) {
        None => Resposta::NaoEncontrado(
            String::from("{ \"mensagem\": \"Produto não encontrado\" }")),
        Some(p) => Resposta::Ok(serde_json::to_string(&p).unwrap()),
    }
}

#[delete("/<prod_id>")]
fn deleta(pool: &State<ConexaoPool>, prod_id: i32) -> Resposta {
    let conexao = pool.get().unwrap();
    match produtos::get_produto(&conexao, prod_id) {
        None => Resposta::NaoEncontrado(
            String::from("{ \"mensagem\": \"Produto não encontrado\" }")),
        Some(_) => {
            produtos::deleta_produto(&conexao, prod_id);
            Resposta::Ok(format!("{{ \"id\": {} }}", prod_id))
        }
    }
}

#[delete("/all")]
fn deleta_todos(pool: &State<ConexaoPool>) -> Resposta {
    use crate::model::schema::produto;
    let conexao = pool.get().unwrap();
    let num_deletados = diesel::delete(produto::table)
        .execute(&conexao)
        .expect("Erro ao deletar endereços");
    Resposta::Ok(format!("{{ \"produtos\": {} }}", num_deletados))
}

#[post("/", data = "<dados>")]
fn cadastra(pool: &State<ConexaoPool>, dados: Json<ProdutoRecv>) -> Resposta {
    let conexao = pool.get().unwrap();
    let result = produtos::registra_produto(&conexao, dados.clone());
    match result {
        Ok(id) => Resposta::Ok(format!("{{ \"id\": {} }}", id)),
        Err(msg) =>
            Resposta::ErroSemantico(format!("{{ \"mensagem\": \"{}\" }}", msg)),
    }
}

#[post("/<prod_id>/mov_estoque", data = "<dados>")]
fn movimenta_estoque(
    pool: &State<ConexaoPool>,
    prod_id: i32,
    dados: Json<MovEstoque>
) -> Resposta {
    let conexao = pool.get().unwrap();
    if produtos::get_produto(&conexao, prod_id).is_none() {
        Resposta::NaoEncontrado(
            String::from("{ \"mensagem\": \"Produto não encontrado\" }"))
    } else {
        match produtos::muda_estoque(&conexao, dados.clone()) {
            Ok(estoque) => Resposta::Ok(
                format!("{{ \"estoque\": \"{}\" }}", estoque)),
            Err(msg) => Resposta::ErroSemantico(
                format!("{{ \"mensagem\": \"{}\" }}", msg)),
        }
    }
}
