// db/redis.rs -- Uma parte de Minerva.rs
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

use r2d2_redis::{ r2d2, RedisConnectionManager };
use std::env;
use dotenv::dotenv;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

pub fn cria_pool_redis() -> RedisPool {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL")
        .expect("Necessário definir o URL do Redis em REDIS_URL");

    let manager = RedisConnectionManager::new(redis_url)
        .expect("Falha ao criar gerente de conexões do Redis.");

    r2d2::Pool::builder()
        .build(manager)
        .expect("Falha ao criar pool do Redis.")
}
