// model/enum_error.rs -- Uma parte de Minerva.rs
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

//! Utilitários para definição de erros de decodificação em `enums`.
//!
//! Estes utilitários foram criados para que certos números sejam
//! automaticamente decodificados, em estruturas que os representam no banco de
//! dados, como enumerações.
//! Para exemplos, veja a implementação de
//! [`DBOperacao`][`super::logdb::DBOperacao`].

#![allow(dead_code)]

/// Representa um erro durante a decodificação de um `Enum`.
#[derive(Debug)]
pub struct EnumError {
    mensagem: String,
    status: u16,
}

impl EnumError {
    /// Cria um erro de decodificação de um `Enum` tal que o mesmo retorne
    /// uma mensagem de status HTTP de "não encontrado".
    pub fn nao_encontrado(mensagem: String) -> Self {
        Self {
            mensagem,
            status: 404,
        }
    }
}
