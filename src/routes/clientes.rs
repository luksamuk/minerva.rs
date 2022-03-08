// routes/clientes.rs -- Uma parte de Minerva.rs
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

//! Rotas para requisições envolvendo dados de clientes.

use super::respostas::Resposta;
use crate::auth::AuthKey;
use crate::bo;
use crate::controller::clientes;
use crate::db::ConexaoPool;
use crate::model::cliente::ClienteRecv;
use rocket::serde::json::Json;
use rocket::Route;
use rocket::State;
use serde_json::json;

/// Constrói as subrotas da rota `/clientes`.
///
/// As rotas construídas estão listadas a seguir:
/// - `GET /` (requer autenticação);
/// - `GET /<id>` (requer autenticação);
/// - `POST /` (requer autenticação);
/// - `DELETE /<id>` (requer autenticação);
/// - `DELETE /all` (requer autenticação).
pub fn constroi_rotas() -> Vec<Route> {
    routes![index, deleta_todos, retorna_usuario, cadastra, deleta]
}

#[get("/")]
fn index(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    let vec_clientes = clientes::lista_clientes(&conexao, 100);
    Resposta::Ok(serde_json::to_string(&vec_clientes).unwrap())
}

#[get("/<ident>")]
fn retorna_usuario(pool: &State<ConexaoPool>, ident: i32, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    match clientes::get_cliente(&conexao, ident) {
        None => Resposta::NaoEncontrado(
            json!({
                "mensagem": "Cliente não encontrado"
            })
            .to_string(),
        ),
        Some(c) => Resposta::Ok(serde_json::to_string(&c).unwrap()),
    }
}

#[post("/", data = "<dados>")]
fn cadastra(pool: &State<ConexaoPool>, dados: Json<ClienteRecv>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    if let Err(s) = bo::clientes::valida_dados(&dados) {
        Resposta::ErroSemantico(s)
    } else {
        let id = clientes::registra_cliente(&conexao, dados.clone());
        Resposta::Ok(json!({ "id": id }).to_string())
    }
}

#[delete("/<ident>")]
fn deleta(pool: &State<ConexaoPool>, ident: i32, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    match clientes::get_cliente(&conexao, ident) {
        None => Resposta::NaoEncontrado(
            json!({
                "mensagem": "Cliente não encontrado"
            })
            .to_string(),
        ),
        Some(c) => {
            let id = c.id;
            clientes::deleta_cliente(&conexao, c);
            Resposta::Ok(json!({ "id": id }).to_string())
        }
    }
}

#[delete("/all")]
fn deleta_todos(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    let (num_end, num_cl) = clientes::deleta_todos(&conexao);
    Resposta::Ok(
        json!({
        "clientes": num_cl,
        "enderecos": num_end
        })
        .to_string(),
    )
}
