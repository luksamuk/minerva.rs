// controller/log.rs -- Uma parte de Minerva.rs
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

use diesel::prelude::*;
use crate::model::logdb::{ LogDB, NovoLogDB, DBOperacao };
use crate::model::schema::logdb;
use std::time::SystemTime;

pub fn registra_log(
    conexao: &PgConnection,
    tabela: String,
    usuario: String,
    operacao: DBOperacao,
    descricao: Option<String>
) -> i32 {
    let log = NovoLogDB {
        tabela,
        usuario,
        operacao,
        datahora: SystemTime::now(),
        descricao
    };
    diesel::insert_into(logdb::table)
        .values(&log)
        .get_result::<LogDB>(conexao)
        .unwrap()
        .id
}
