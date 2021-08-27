// model/produto.rs -- Uma parte de Minerva.rs
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

use super::schema::produto;
use serde::{ Serialize, Deserialize };
use bigdecimal::BigDecimal;
use std::str::FromStr;

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Produto {
    pub id: i32,
    pub descricao: String,
    pub unidsaida: String,
    pub qtdestoque: BigDecimal,
    pub precovenda: BigDecimal,
}

#[derive(Debug, Insertable, Deserialize, Clone)]
#[table_name="produto"]
pub struct NovoProduto {
    pub descricao: String,
    pub unidsaida: String,
    pub qtdestoque: BigDecimal,
    pub precovenda: BigDecimal,
}

impl NovoProduto {
    pub fn new() -> Self {
        Self {
            descricao: String::new(),
            unidsaida: String::new(),
            qtdestoque: BigDecimal::from_str("0.0").unwrap(),
            precovenda: BigDecimal::from_str("0.0").unwrap(),
        }
    }
}
