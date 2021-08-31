// db/postgres.rs -- Uma parte de Minerva.rs
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

pub fn cria_pool_conexoes() -> ConexaoPool {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("Necessário definir o URL do BD em DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder()
        .build(manager)
        .expect("Falha ao criar pool de conexões.")
}

pub fn garante_usuario_inicial(pool: &ConexaoPool) {
    use crate::controller::usuarios;
    use crate::model::usuario::{ UsuarioRecv, NovoUsuario };
    use diesel::prelude::*;
    use crate::model::schema::usuario;
    
    let conexao = pool.get().unwrap();
    if usuarios::lista_usuarios(&conexao, 1).is_empty() {
        let novo_admin = NovoUsuario::from(&UsuarioRecv {
            login: String::from("admin"),
            nome:  String::from("Admin"),
            email: None,
            senha: String::from("admin"),
        });
        let _ = diesel::insert_into(usuario::table)
            .values(&novo_admin)
            .execute(&conexao)
            .expect("Erro ao cadastrar usuário \"admin\"");
    }
}
