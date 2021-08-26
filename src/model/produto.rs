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
use diesel::pg::data_types::PgNumeric;
use serde::{ Serialize, Deserialize };
use crate::util::*;

#[derive(Queryable, Serialize, Debug)]
pub struct Produto {
    pub id: i32,
    pub descricao: String,
    pub unidsaida: String,
    #[serde(with = "Numeric")]
    pub qtdestoque: PgNumeric,
    #[serde(with = "Numeric")]
    pub precovenda: PgNumeric
}

#[derive(Insertable)]
#[table_name="produto"]
pub struct NovoProduto {
    pub descricao: String,
    pub unidsaida: String,
    pub qtdestoque: PgNumeric,
    pub precovenda: PgNumeric,
}

#[derive(Serialize)]
pub struct ProdutoRepr {
    pub id: i32,
    pub descricao: String,
    pub unidsaida: String,
    pub qtdestoque: String,
    pub precovenda: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProdutoRecv {
    pub descricao: String,
    pub unidsaida: String,
    pub qtdestoque: String,
    pub precovenda: String,
}

impl Produto {
    pub fn into_repr(&self) -> ProdutoRepr {
        ProdutoRepr {
            id: self.id,
            descricao: self.descricao.clone(),
            unidsaida: self.unidsaida.clone(),
            qtdestoque: numeric_to_string(self.qtdestoque.clone()),
            precovenda: numeric_to_string(self.precovenda.clone()),
        }
    }
}

impl NovoProduto {
    pub fn new() -> Self {
        Self {
            descricao: String::new(),
            unidsaida: String::new(),
            qtdestoque: PgNumeric::NaN,
            precovenda: PgNumeric::NaN,
        }
    }
}

impl ProdutoRecv {
    pub fn into_new(&self) -> NovoProduto {
        NovoProduto {
            descricao: self.descricao.clone(),
            unidsaida: self.unidsaida.clone(),
            qtdestoque: string_to_numeric(self.qtdestoque.clone()),
            precovenda: string_to_numeric(self.precovenda.clone()),
        }
    }
}
