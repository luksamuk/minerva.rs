// bo/usuarios.rs -- Uma parte de Minerva.rs
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

//! Este módulo contém ferramentas para reforçar regras de negócio relacionadas
//! à validação de transações envolvendo dados de usuários do sistema.

use sodiumoxide::crypto::pwhash::argon2id13;

/// Compara por igualdade uma senha fornecida em texto e a senha de um usuário,
/// utilizando o hash da senha do usuário referido.
/// 
/// A senha deverá fornecida como um slice de String, e o hash da senha deverá
/// ser uma referência direta aos bytes da senha com hash, da forma armazenada
/// no banco de dados, envolvendo o hash e o salt para aquele usuário.
pub fn verifica_senha(senha: &str, senha_hash: &[u8]) -> bool {
    sodiumoxide::init().unwrap();
    match argon2id13::HashedPassword::from_slice(senha_hash) {
        Some(hp) => argon2id13::pwhash_verify(&hp, senha.as_bytes()),
        None => false,
    }
}
