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
use crate::bo::db::ConexaoPool;
use crate::controller::estoque;
use crate::model::estoque::{Estoque, MovEstoqueRecv};
use rocket::serde::json::Json;
use rocket::Route;
use rocket::State;
use serde_json::json;

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
        mostra_entradas,
        mostra_saidas,
    ]
}

#[get("/<prod_id>")]
async fn mostra_estoque(pool: &State<ConexaoPool>, prod_id: i32, _auth: AuthKey<'_>) -> Resposta {
    match estoque::mostra_estoque(pool, prod_id).await {
        None => Resposta::NaoEncontrado(
            json!({
                "mensagem": "Produto não encontrado"
            })
            .to_string(),
        ),
        Some(e) => Resposta::Ok(serde_json::to_string(&e).unwrap()),
    }
}

#[get("/")]
async fn lista_estoque(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let lista = estoque::lista_estoque(pool, 100).await;
    Resposta::Ok(serde_json::to_string(&lista).unwrap())
}

#[post("/", data = "<dados>")]
async fn inicia_estoque(
    pool: &State<ConexaoPool>,
    dados: Json<Estoque>,
    _auth: AuthKey<'_>,
) -> Resposta {
    estoque::inicia_estoque(pool, dados.clone()).await
}

#[post("/mov", data = "<dados>")]
async fn movimenta_estoque(
    pool: &State<ConexaoPool>,
    dados: Json<MovEstoqueRecv>,
    _auth: AuthKey<'_>,
) -> Resposta {
    estoque::movimenta_estoque(pool, dados.clone()).await
}

#[get("/mov")]
async fn mostra_movimentos(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    Resposta::Ok(serde_json::to_string(&estoque::recupera_movimentos(pool, 100).await).unwrap())
}

#[get("/mov/entradas")]
async fn mostra_entradas(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    Resposta::Ok(
        serde_json::to_string(&estoque::recupera_movimentos_filtrado(pool, 100, true).await)
            .unwrap(),
    )
}

#[get("/mov/saidas")]
async fn mostra_saidas(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    Resposta::Ok(
        serde_json::to_string(&estoque::recupera_movimentos_filtrado(pool, 100, false).await)
            .unwrap(),
    )
}
