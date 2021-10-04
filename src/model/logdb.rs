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

//! Utilitários de modelagem de entradas de log para banco de dados e regras de
//! negócio.
//! 
//! Este módulo define estruturas para criação e recuperação de logs no sistema.
//! Os logs serão sempre armazenados na tabela `logdb`.

use super::enum_error::EnumError;
use super::schema::logdb;
use chrono::DateTime;
use diesel::sql_types::SmallInt;
use diesel_enum::DbEnum;
use num_derive::FromPrimitive;
use serde::Serialize;
use serde_repr::Serialize_repr;

/// Representa um tipo de operação de modificação de estado nas tabelas do banco
/// de dados.
/// 
/// Este tipo de operação deverá ser armazenado para descrever a natureza da
/// operação realizada na tabela a ela relacionada.
#[derive(
    FromPrimitive,
    ToPrimitive,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    AsExpression,
    FromSqlRow,
    DbEnum,
    Serialize_repr,
)]
#[sql_type = "SmallInt"]
#[error_fn = "EnumError::nao_encontrado"]
#[error_type = "EnumError"]
#[repr(i16)]
pub enum DBOperacao {
    /// Operação de inserção em uma tabela do banco de dados.
    Insercao = 0,
    /// Operação de alteração de um registro em uma tabela do banco de dados.
    Alteracao = 1,
    /// Operação de remoção de um registro em uma tabela do banco de dados.
    Remocao = 2,
}

/// Representa um único registro de log para uma operação no banco de dados.
/// 
/// O registro armazena dados relacionados a uma única operação de natureza
/// específica, realizada por um usuário específico e em uma tabela específica
/// do banco de dados.
#[derive(Queryable, Serialize, Clone)]
pub struct LogDB {
    /// Id do registro de log no banco de dados.
    pub id: i32,
    /// Tabela onde a operação se deu.
    pub tabela: String,
    /// Usuário que realizou a operação.
    pub usuario: String,
    /// Natureza da operação realizada.
    pub operacao: DBOperacao,
    /// Data e hora de registro do log da operação.
    pub datahora: DateTime<chrono::Utc>,
    /// Descrição da operação efetuada. Não é necessário informar.
    pub descricao: Option<String>,
}

/// Representa uma estrutura de inserção de um novo registro de log de uma
/// operação no banco de dados.
#[derive(Insertable)]
#[table_name = "logdb"]
pub struct NovoLogDB {
    /// Tabela onde a operação se deu.
    /// Veja [`LogDB::tabela`].
    pub tabela: String,
    /// Usuário que realizou a operação.
    /// Veja [`LogDB::usuario`].
    pub usuario: String,
    /// Natureza da operação realizada.
    /// Veja [`LogDB::operacao`].
    pub operacao: DBOperacao,
    /// Data e hora de registro do log da operação.
    /// Veja [`LogDB::datahora`].
    pub datahora: DateTime<chrono::Utc>,
    /// Descrição opcional da operação efetuada.
    /// Veja [`LogDB::descricao`].
    pub descricao: Option<String>,
}
