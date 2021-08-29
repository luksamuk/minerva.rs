// model/logdb.rs -- Uma parte de Minerva.rs
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

use std::time::SystemTime;
use super::schema::logdb;
use num_derive::FromPrimitive;
use diesel_enum::DbEnum;
use diesel::sql_types::SmallInt;
use super::enum_error::EnumError;

#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, DbEnum)]
#[sql_type = "SmallInt"]
#[error_fn = "EnumError::nao_encontrado"]
#[error_type = "EnumError"]
pub enum DBOperacao {
    Insercao  = 0,
    Alteracao = 1,
    Remocao   = 2,
}

#[derive(Queryable)]
pub struct LogDB {
    pub id: i32,
    pub tabela: String,
    pub usuario: String,
    pub operacao: DBOperacao,
    pub datahora: SystemTime,
    pub descricao: Option<String>,
}

#[derive(Insertable)]
#[table_name="logdb"]
pub struct NovoLogDB {
    pub tabela: String,
    pub usuario: String,
    pub operacao: DBOperacao,
    pub datahora: SystemTime,
    pub descricao: Option<String>,
}

