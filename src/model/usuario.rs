// model/usuario.rs -- Uma parte de Minerva.rs
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

use crate::model::schema::usuario;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::pwhash::argon2id13;

#[derive(Queryable, Serialize, Clone)]
pub struct Usuario {
    pub id: i32,
    pub login: String,
    pub nome: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub senha_hash: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "usuario"]
pub struct NovoUsuario {
    pub login: String,
    pub nome: String,
    pub email: Option<String>,
    pub senha_hash: Vec<u8>,
}

#[derive(Deserialize, Clone)]
pub struct UsuarioRecv {
    pub login: String,
    pub nome: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub senha: String,
}

impl From<&UsuarioRecv> for NovoUsuario {
    fn from(usr: &UsuarioRecv) -> Self {
        sodiumoxide::init().unwrap();
        let hash = argon2id13::pwhash(
            usr.senha.trim().as_bytes(),
            argon2id13::OPSLIMIT_INTERACTIVE,
            argon2id13::MEMLIMIT_INTERACTIVE,
        )
        .unwrap();

        NovoUsuario {
            login: usr.login.clone().trim().to_string(),
            nome: usr.nome.clone().trim().to_string(),
            email: usr.email.clone(),
            senha_hash: Vec::from(hash.0),
        }
    }
}

impl Usuario {
    pub fn verifica_senha(&self, senha: &str) -> bool {
        sodiumoxide::init().unwrap();
        match argon2id13::HashedPassword::from_slice(&self.senha_hash) {
            Some(hp) => argon2id13::pwhash_verify(&hp, senha.as_bytes()),
            None => false,
        }
    }
}
