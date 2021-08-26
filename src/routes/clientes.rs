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

use rocket::Route;
use rocket::serde::json::Json;
use crate::controller::clientes;
use rocket::State;
use crate::db::ConexaoPool;
use diesel::prelude::*;
use crate::model::cliente::ClienteRecv;
use super::respostas::Resposta;

pub fn constroi_rotas() -> Vec<Route> {
    routes![
        index,
        deleta_todos,
        retorna_usuario,
        cadastra,
        deleta
    ]
}

#[get("/")]
fn index(pool: &State<ConexaoPool>) -> Resposta {
    let conexao = pool.get().unwrap();
    let vec_clientes = clientes::lista_clientes(&conexao, 100);
    Resposta::Ok(serde_json::to_string(&vec_clientes).unwrap())
}

#[get("/<ident>")]
fn retorna_usuario(pool: &State<ConexaoPool>, ident: i32) -> Resposta {
    let conexao = pool.get().unwrap();
    match clientes::get_cliente(&conexao, ident) {
        None => Resposta::NaoEncontrado(
            "{ \"mensagem\": \"Cliente não encontrado\" }".to_string()),
        Some(c) => Resposta::Ok(serde_json::to_string(&c).unwrap())
    }
}

#[post("/", data = "<dados>")]
fn cadastra(pool: &State<ConexaoPool>, dados: Json<ClienteRecv>) -> Resposta {
    let conexao = pool.get().unwrap();
    let id = clientes::registra_cliente(&conexao, dados.clone());
    Resposta::Ok(format!("{{ \"id\": {} }}", id))
}

#[delete("/<ident>")]
fn deleta(pool: &State<ConexaoPool>, ident: i32) -> Resposta {
    let conexao = pool.get().unwrap();
    match clientes::get_cliente(&conexao, ident) {
        None => Resposta::NaoEncontrado(
            "{ \"mensagem\": \"Cliente não encontrado\" }".to_string()),
        Some(c) => {
            let id = c.id;
            clientes::deleta_cliente(&conexao, c);
            Resposta::Ok(format!("{{ \"id\": {} }}", id))
        }
    }
}

#[delete("/all")]
fn deleta_todos(pool: &State<ConexaoPool>) -> Resposta {
    use crate::model::schema::{ cliente, endereco };
    let conexao = pool.get().unwrap();
    let num_end_deletados = diesel::delete(endereco::table)
        .execute(&conexao)
        .expect("Erro ao deletar endereços");    
    let num_deletados = diesel::delete(cliente::table)
        .execute(&conexao)
        .expect("Erro ao deletar clientes");
    Resposta::Ok(format!("{{ \"clientes\": {}, \"enderecos\": {} }}",
                         num_deletados, num_end_deletados))
}

