// routes/login.rs -- Uma parte de Minerva.rs
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

use rocket::{ State, Route };
use rocket::serde::json::Json;
use crate::db::{ ConexaoPool, RedisPool };
use crate::model::login::LoginData;
use super::respostas::Resposta;
use crate::controller::login;

pub fn constroi_rotas() -> Vec<Route> {
    routes![realiza_login]
}

#[post("/", data = "<dados>")]
fn realiza_login(pool: &State<ConexaoPool>, redispool: &State<RedisPool>, dados: Json<LoginData>) -> Resposta {
    let conexao = pool.get().unwrap();
    let mut redis = redispool.get().unwrap();
    login::loga_usuario(&conexao, &mut redis, &dados)
}
