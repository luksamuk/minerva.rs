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

use dotenv::dotenv;
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::env;

/// Representa uma pool de conexões para o Redis.
pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

/// Representa uma conexão com o serviço Redis.
pub type RedisConnection = diesel::r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>;

/// Cria um pool de conexões com o serviço Redis.
/// Deve ser chamada uma vez ao início da aplicação.
///
/// O URL para o serviço do Redis pode ser definido através da variável de
/// ambiente `REDIS_URL`, que também pode ser definida em um arquivo `.env`
/// no diretório em que a aplicação for iniciada, com a seguinte formatação:
///
/// `REDIS_URL=redis://localhost:6379`
///
/// # Panics
///
/// A função entrará em pânico se a variável `REDIS_URL` não for definida no
/// ambiente, e se algum erro anormal ocorrer ao criar a pool ou seu gerente
/// de conexões subjacente.
///
/// # Exemplo
///
/// O exemplo a seguir cria um pool, recupera uma conexão com o Redis e lê
/// o conteúdo de uma variável de nome `variavel`, assumindo que a mesma
/// contenha uma string.
///
/// ```
/// use minerva::db::redis::*;
///
/// let redis_pool = cria_pool_redis();
/// let mut redis = redis_pool.get().unwrap();
/// let texto: String = redis.get("variavel").unwrap();
/// ```
/// A conexão obtida será devolvida ao pool ao sair do escopo atual.
pub fn cria_pool_redis() -> RedisPool {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("Necessário definir o URL do Redis em REDIS_URL");

    let manager = RedisConnectionManager::new(redis_url)
        .expect("Falha ao criar gerente de conexões do Redis.");

    r2d2::Pool::builder()
        .build(manager)
        .expect("Falha ao criar pool do Redis.")
}
