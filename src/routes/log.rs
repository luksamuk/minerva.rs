// routes/log.rs -- Uma parte de Minerva.rs
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

//! Rotas para requisições envolvendo dados de log do sistema.

use super::respostas::Resposta;
use crate::bo::auth::AuthKey;
use crate::bo::db::ConexaoPool;
use crate::controller::log;
use rocket::{Route, State};

/// Constrói as subrotas da rota `/log`.
///
/// As rotas construídas estão listadas a seguir:
/// - `GET /` (requer autenticação);
/// - `GET /txt` (texto plano -- requer autenticação).
pub fn constroi_rotas() -> Vec<Route> {
    routes![mostra_log, mostra_log_texto]
}

#[get("/")]
fn mostra_log(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    Resposta::Ok(serde_json::to_string(&log::recupera_log(&conexao, 100)).unwrap())
}

#[get("/txt")]
fn mostra_log_texto(pool: &State<ConexaoPool>, _auth: AuthKey<'_>) -> Resposta {
    let conexao = pool.get().unwrap();
    Resposta::OkTexto(log::lista_log_texto(&conexao))
}
