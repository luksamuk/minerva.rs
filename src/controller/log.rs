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

pub use crate::model::logdb::DBOperacao;
use crate::model::logdb::{LogDB, NovoLogDB};
use crate::model::schema::logdb;
use diesel::prelude::*;

pub fn registra_log(
    conexao: &PgConnection,
    tabela: String,
    usuario: String,
    operacao: DBOperacao,
    descricao: Option<String>,
) -> i32 {
    diesel::insert_into(logdb::table)
        .values(&NovoLogDB {
            tabela,
            usuario,
            operacao,
            datahora: chrono::offset::Utc::now(),
            descricao,
        })
        .get_result::<LogDB>(conexao)
        .unwrap()
        .id
}

pub fn lista_log_texto(conexao: &PgConnection) -> String {
    use comfy_table::presets::ASCII_BORDERS_ONLY_CONDENSED;
    use comfy_table::Table;
    let logs = recupera_log(conexao, 100);
    let mut table = Table::new();
    table
        .load_preset(ASCII_BORDERS_ONLY_CONDENSED)
        .set_header(vec![
            "Tabela",
            "Usuário",
            "Operação",
            "Data/Hora",
            "Descrição",
        ]);
    for log in logs {
        table.add_row(vec![
            log.tabela.clone(),
            log.usuario.clone(),
            String::from(match log.operacao {
                DBOperacao::Insercao => "Inserção",
                DBOperacao::Alteracao => "Alteração",
                DBOperacao::Remocao => "Remoção",
            }),
            format!("{}", log.datahora),
            log.descricao.unwrap_or(String::new()),
        ]);
    }
    format!("{}\n", table)
}

pub fn recupera_log(conexao: &PgConnection, limite: i64) -> Vec<LogDB> {
    use crate::model::schema::logdb::dsl::*;
    logdb
        .order(datahora.desc())
        .limit(limite)
        .load::<LogDB>(conexao)
        .expect("Erro ao recuperar logs")
}
