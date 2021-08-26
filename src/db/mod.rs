// db/mod.rs -- Uma parte de Minerva.rs
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

use diesel::r2d2::{ ConnectionManager, Pool };
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

pub type ConexaoPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> ConexaoPool {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("Necessário definir o URL do BD em DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder()
        .build(manager)
        .expect("Falha ao criar pool de conexões.")
}
