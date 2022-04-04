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

//! Rotas para requisições envolvendo manipulação de produtos.

use super::respostas::Resposta;
use crate::bo::auth::AuthKey;
use crate::controller::produtos;
use crate::bo::db::ConexaoPool;
use crate::model::produto::NovoProduto;
use rocket::serde::json::Json;
use rocket::Route;
use rocket::State;
use serde_json::json;

/// Constrói as subrotas da rota `/produtos`.
///
/// As rotas construídas estão listadas a seguir:
/// - `GET /` (requer autenticação);
/// - `POST /` (requer autenticação);
/// - `GET /<id>` (requer autenticação);
/// - `DELETE /<id>` (requer autenticação);
/// - `DELETE /all` (requer autenticação).
pub fn constroi_rotas() -> Vec<Route> {
    routes![index, retorna_produto, deleta_todos, deleta, cadastra]
}

#[get("/")]
fn index(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    let vec_produtos = produtos::lista_produtos(&conexao, 100);
    Resposta::Ok(serde_json::to_string(&vec_produtos).unwrap())
}

#[get("/<prod_id>")]
fn retorna_produto(pool: &State<ConexaoPool>, prod_id: i32, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    match produtos::get_produto(&conexao, prod_id) {
        None => Resposta::NaoEncontrado(
            json!({
                "mensagem": "Produto não encontrado"
            })
            .to_string(),
        ),
        Some(p) => Resposta::Ok(serde_json::to_string(&p).unwrap()),
    }
}

#[delete("/<prod_id>")]
fn deleta(pool: &State<ConexaoPool>, prod_id: i32, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    match produtos::get_produto(&conexao, prod_id) {
        None => Resposta::NaoEncontrado(
            json!({
                "mensagem": "Produto não encontrado"
            })
            .to_string(),
        ),
        Some(_) => {
            produtos::deleta_produto(&conexao, prod_id);
            Resposta::Ok(json!({ "id": prod_id }).to_string())
        }
    }
}

#[delete("/all")]
fn deleta_todos(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    let num_del = produtos::deleta_todos(&conexao);
    Resposta::Ok(json!({ "produtos": num_del }).to_string())
}

#[post("/", data = "<dados>")]
fn cadastra(pool: &State<ConexaoPool>, dados: Json<NovoProduto>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    let result = produtos::registra_produto(&conexao, dados.clone());
    match result {
        Ok(id) => Resposta::Ok(json!({ "id": id }).to_string()),
        Err(msg) => Resposta::ErroSemantico(json!({ "mensagem": msg }).to_string()),
    }
}
