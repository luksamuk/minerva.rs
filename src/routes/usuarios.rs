// routes/usuarios.rs -- Uma parte de Minerva.rs
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

//! Rotas para requisições envolvendo manipulação de usuários do sistema.
//! 
//! As rotas de usuários não envolvem [login][`super::login`], apenas cadastro e
//! visualização de dados de usuários do sistema.

use super::respostas::Resposta;
use crate::bo::auth::AuthKey;
use crate::controller::usuarios;
use crate::db::ConexaoPool;
use crate::model::usuario::UsuarioRecv;
use rocket::serde::json::Json;
use rocket::{Route, State};

/// Constrói as subrotas da rota `/estoque`.
/// 
/// As rotas construídas estão listadas a seguir:
/// - `GET /` (requer autenticação);
/// - `POST /` (requer autenticação);
/// - `GET /<id>` (requer autenticação);
/// - `GET /<login>` (requer autenticação);
/// - `DELETE /<id>` (requer autenticação);
/// - `DELETE /<login>` (requer autenticação).
pub fn constroi_rotas() -> Vec<Route> {
    routes![
        index,
        retorna_por_id,
        retorna_por_login,
        cadastra,
        deleta_por_id,
        deleta_por_login,
    ]
}

#[get("/")]
fn index(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    let vec_usuarios = usuarios::lista_usuarios(&conexao, 100);
    Resposta::Ok(serde_json::to_string(&vec_usuarios).unwrap())
}

#[get("/<usr_id>")]
fn retorna_por_id(pool: &State<ConexaoPool>, usr_id: i32, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    match usuarios::get_usuario(&conexao, usr_id) {
        None => {
            Resposta::NaoEncontrado(String::from("{ \"mensagem\": \"Usuário não encontrado\" }"))
        }
        Some(u) => Resposta::Ok(serde_json::to_string(&u).unwrap()),
    }
}

#[get("/<usr_login>", rank = 2)]
fn retorna_por_login(pool: &State<ConexaoPool>, usr_login: &str, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    match usuarios::encontra_usuario(&conexao, usr_login) {
        None => {
            Resposta::NaoEncontrado(String::from("{ \"mensagem\": \"Usuário não encontrado\" }"))
        }
        Some(u) => Resposta::Ok(serde_json::to_string(&u).unwrap()),
    }
}

#[post("/", data = "<dados>")]
fn cadastra(pool: &State<ConexaoPool>, dados: Json<UsuarioRecv>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();

    if usuarios::encontra_usuario(&conexao, dados.login).is_some() {
        return Resposta::ErroSemantico(format!(
            "{{ \"mensagem\": \"O nome de usuário \\\"{}\\\" já existe\" }}",
            dados.login
        ));
    }

    let result = usuarios::registra_usuario(&conexao, &dados);
    match result {
        Ok((id, login)) => Resposta::Ok(format!("{{ \"id\": {}, \"login\": \"{}\" }}", id, login)),
        Err(msg) => Resposta::ErroSemantico(format!("{{ \"mensagem\": \"{}\" }}", msg)),
    }
}

#[delete("/<usr_id>")]
fn deleta_por_id(pool: &State<ConexaoPool>, usr_id: i32, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    match usuarios::get_usuario(&conexao, usr_id) {
        None => {
            Resposta::NaoEncontrado(String::from("{ \"mensagem\": \"Usuário não encontrado\" }"))
        }
        Some(u) => {
            usuarios::deleta_usuario_por_id(&conexao, usr_id);
            Resposta::Ok(format!(
                "{{ \"id\": {}, \"login\": \"{}\" }}",
                usr_id, u.login
            ))
        }
    }
}

#[delete("/<usr_login>", rank = 2)]
fn deleta_por_login(pool: &State<ConexaoPool>, usr_login: &str, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    match usuarios::encontra_usuario(&conexao, usr_login) {
        None => {
            Resposta::NaoEncontrado(String::from("{ \"mensagem\": \"Usuário não encontrado\" }"))
        }
        Some(u) => {
            usuarios::deleta_usuario_por_id(&conexao, u.id);
            Resposta::Ok(format!(
                "{{ \"id\": {}, \"login\": \"{}\" }}",
                u.id, u.login
            ))
        }
    }
}
