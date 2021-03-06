// main.rs -- Uma parte de Minerva.rs
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

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

use diesel_migrations::embed_migrations;
use dotenv::dotenv;
use minerva::*;

embed_migrations!();

#[launch]
fn launch() -> _ {
    println!("Iniciando Minerva REST Server...");
    dotenv().ok();
    dotenv::from_filename(".env.local").ok();

    let pool = bo::db::cria_pool_conexoes();
    bo::db::executa_migrations(&pool);
    bo::db::garante_usuario_inicial(&pool);

    let redis_pool = bo::redis::cria_pool_redis();

    //let twilio = bo::twilio::cria_conexao_twilio();

    rocket::build()
        .manage(pool)
        .manage(redis_pool)
        //.manage(twilio.ok())
        .mount("/", routes![routes::index])
        .mount("/login", routes::login::constroi_rotas())
        .mount("/clientes", routes::clientes::constroi_rotas())
        .mount("/produtos", routes::produtos::constroi_rotas())
        .mount("/estoque", routes::estoque::constroi_rotas())
        .mount("/log", routes::log::constroi_rotas())
        .mount("/usuarios", routes::usuarios::constroi_rotas())
}
