// model/mod.rs -- Uma parte de Minerva.rs
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

//! Utilitários de modelagem de estruturas do banco de dados e outras entidades.
//!
//! Este módulo define estruturas para tráfego de dados entre as rotas,
//! controllers e o banco de dados. Os schemas são automaticamente gerados pelas
//! migrations da biblioteca [Diesel].
//!
//! [Diesel]: https://diesel.rs

pub mod cliente;
pub mod endereco;
pub mod enum_error;
pub mod estoque;
pub mod logdb;
pub mod login;
pub mod produto;
#[allow(missing_docs)]
pub mod schema;
pub mod usuario;
