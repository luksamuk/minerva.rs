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

use diesel::pg::data_types::PgNumeric;
use serde::Deserialize;
use crate::util::string_to_numeric;

pub struct MovEstoque {
    pub produto_id: i32,
    pub movimentacao_estoque: PgNumeric,
}

#[derive(Deserialize, Clone)]
pub struct MovEstoqueRecv {
    pub produto_id: i32,
    pub movimentacao_estoque: String,
}

impl MovEstoqueRecv {
    pub fn into_proper(&self) -> MovEstoque {
        MovEstoque {
            produto_id: self.produto_id,
            movimentacao_estoque: string_to_numeric(self.movimentacao_estoque.clone()),
        }
    }
}
