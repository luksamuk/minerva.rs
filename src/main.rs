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

#[macro_use] extern crate rocket;

use minerva::*;

#[launch]
fn launch() -> _ {
    let pool = db::cria_pool_conexoes();
    db::garante_usuario_inicial(&pool);

    let redis_pool = db::cria_pool_redis();

    rocket::build()
        .manage(pool)
        .manage(redis_pool)
        .mount("/", routes![routes::index])
        .mount("/login", routes::login::constroi_rotas())
        .mount("/clientes", routes::clientes::constroi_rotas())
        .mount("/produtos", routes::produtos::constroi_rotas())
        .mount("/estoque", routes::estoque::constroi_rotas())
        .mount("/log", routes::log::constroi_rotas())
        .mount("/usuarios", routes::usuarios::constroi_rotas())
}
