// routes/estoque.rs -- Uma parte de Minerva.rs
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

//! Rotas para requisições envolvendo dados de estoque e movimentação de estoque
//! de produtos.

use super::respostas::Resposta;
use crate::bo::auth::AuthKey;
use crate::controller::estoque;
use crate::db::ConexaoPool;
use crate::model::estoque::{Estoque, MovEstoqueRecv};
use rocket::serde::json::Json;
use rocket::Route;
use rocket::State;

/// Constrói as subrotas da rota `/estoque`.
/// 
/// As rotas construídas estão listadas a seguir:
/// 
/// ## Rotas de posição de estoque
/// - `GET /` (requer autenticação);
/// - `GET /<id>` (requer autenticação);
/// - `POST /` (requer autenticação);
/// 
/// ## Rotas de movimentação de estoque
/// - `GET /mov` (requer autenticação);
/// - `POST /mov` (requer autenticação);
/// - `GET /mov/entradas` (requer autenticação);
/// - `GET /mov/saidas` (requer autenticação);
/// - `GET /mov/txt` (texto plano -- requer autenticação);
/// - `GET /mov/entradas/txt` (texto plano -- requer autenticação);
/// - `GET /mov/saidas/txt` (texto plano -- requer autenticação).
pub fn constroi_rotas() -> Vec<Route> {
    routes![
        inicia_estoque,
        lista_estoque,
        mostra_estoque,
        movimenta_estoque,
        mostra_movimentos,
        mostra_movimentos_txt,
        mostra_entradas,
        mostra_entradas_txt,
        mostra_saidas,
        mostra_saidas_txt,
    ]
}

#[get("/<prod_id>")]
fn mostra_estoque(pool: &State<ConexaoPool>, prod_id: i32, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    match estoque::mostra_estoque(&conexao, prod_id) {
        None => {
            Resposta::NaoEncontrado(String::from("{ \"mensagem\": \"Produto não encontrado\" }"))
        }
        Some(e) => Resposta::Ok(serde_json::to_string(&e).unwrap()),
    }
}

#[get("/")]
fn lista_estoque(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    let lista = estoque::lista_estoque(&conexao, 100);
    Resposta::Ok(serde_json::to_string(&lista).unwrap())
}

#[post("/", data = "<dados>")]
fn inicia_estoque(pool: &State<ConexaoPool>, dados: Json<Estoque>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    estoque::inicia_estoque(&conexao, dados.clone())
}

#[post("/mov", data = "<dados>")]
fn movimenta_estoque(
    pool: &State<ConexaoPool>,
    dados: Json<MovEstoqueRecv>,
    _auth: AuthKey<'_>,
) -> Resposta {
    let conexao = pool.get().unwrap();
    estoque::movimenta_estoque(&conexao, dados.clone())
}

#[get("/mov")]
fn mostra_movimentos(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    Resposta::Ok(serde_json::to_string(&estoque::recupera_movimentos(&conexao, 100)).unwrap())
}

#[get("/mov/entradas")]
fn mostra_entradas(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    Resposta::Ok(
        serde_json::to_string(&estoque::recupera_movimentos_filtrado(&conexao, 100, true)).unwrap(),
    )
}

#[get("/mov/saidas")]
fn mostra_saidas(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    Resposta::Ok(
        serde_json::to_string(&estoque::recupera_movimentos_filtrado(&conexao, 100, false))
            .unwrap(),
    )
}

#[get("/mov/entradas/txt")]
fn mostra_entradas_txt(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    Resposta::OkTexto(estoque::lista_movimentos_texto_filtrado(
        &conexao, 100, true,
    ))
}

#[get("/mov/saidas/txt")]
fn mostra_saidas_txt(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    Resposta::OkTexto(estoque::lista_movimentos_texto_filtrado(
        &conexao, 100, false,
    ))
}

#[get("/mov/txt")]
fn mostra_movimentos_txt(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    Resposta::OkTexto(estoque::lista_movimentos_texto(&conexao, 100))
}
