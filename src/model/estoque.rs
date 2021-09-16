// model/estoque.rs -- Uma parte de Minerva.rs
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

use bigdecimal::BigDecimal;
use chrono::DateTime;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

use super::schema::{estoque, mov_estoque};

#[derive(Queryable, Insertable, Clone, Identifiable, Serialize, Deserialize)]
#[table_name = "estoque"]
#[primary_key(produto_id)]
pub struct Estoque {
    pub produto_id: i32,
    pub quantidade: BigDecimal,
    pub precounitario: BigDecimal,
}

#[derive(Queryable, Clone, Identifiable, Serialize)]
#[table_name = "mov_estoque"]
pub struct MovEstoque {
    pub id: i32,
    pub produto_id: i32,
    pub docto: String,
    pub quantidade: BigDecimal,
    pub preco_frete: BigDecimal,
    pub datahora: DateTime<chrono::Utc>,
    pub preco_unitario: BigDecimal,
}

#[derive(Insertable, Clone)]
#[table_name = "mov_estoque"]
pub struct NovoMovEstoque {
    pub produto_id: i32,
    pub docto: String,
    pub quantidade: BigDecimal,
    pub preco_unitario: BigDecimal,
    pub preco_frete: BigDecimal,
    pub datahora: DateTime<chrono::Utc>,
}

#[derive(Deserialize, Clone)]
pub struct MovEstoqueRecv {
    pub produto_id: i32,
    pub docto: String,
    pub quantidade: BigDecimal,
    pub preco_unitario: BigDecimal,
    pub preco_frete: Option<BigDecimal>,
}

#[derive(Serialize, Clone)]
pub struct EstoqueUnion {
    pub id: i32,
    pub descricao: String,
    pub unidsaida: String,
    pub quantidade: BigDecimal,
    pub preco_unitario: BigDecimal,
}

impl NovoMovEstoque {
    pub fn from(recv: MovEstoqueRecv) -> Self {
        Self {
            produto_id: recv.produto_id,
            docto: recv.docto.clone(),
            quantidade: recv.quantidade.clone(),
            preco_unitario: recv.preco_unitario.clone(),
            preco_frete: match recv.preco_frete {
                Some(frete) => frete.clone(),
                None => BigDecimal::from_str("0.0000").unwrap(),
            },
            datahora: chrono::offset::Utc::now(),
        }
    }
}
