// controller/mod.rs -- Uma parte de Minerva.rs
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

//! Este módulo contém estruturas e ferramentas para tráfego de dados e
//! comunicação entre as rotas e o banco de dados.
//! 
//! Quando relacionadas à comunicação direta com banco de dados, as regras de
//! negócios poderão ser encontradas no módulo [`bo`][`super::bo`].
//! 
//! Operações de inserção, alteração e inclusão no banco de dados gerarão uma
//! gravação na tabela de log do mesmo.

pub mod clientes;
pub mod estoque;
pub mod log;
pub mod login;
pub mod produtos;
pub mod usuarios;
