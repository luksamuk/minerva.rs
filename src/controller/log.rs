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

//! Ferramentas para escrita de log no banco de dados, envolvendo rotas e os
//! demais controllers.
//!
//! Este módulo descreve funções de registro de log no banco de dados e de
//! listagem de log para o restante do sistema.

pub use crate::model::logdb::DBOperacao;
use crate::model::logdb::{LogDB, NovoLogDB};
use crate::model::schema::logdb;
use diesel::prelude::*;

/// Registra uma movimentação no banco de dados do sistema, usando a tabela de
/// log.
///
/// Esta função auxilia no processo de escrita de log na tabela `logdb`,
/// armazenando o nome da tabela modificada, o usuário a realizar a operação,
/// a operação descrita (alteração, remoção ou inserção) e uma descrição
/// opcional da transação realizada.
///
/// A função retornará o id da linha de log registrada pela função.
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

/// Retorna as últimas cem operações registradas no log, em formato de tabela
/// em texto-plano.
///
/// As operações serão retornadas em ordem decrescente de data de registro, já
/// formatadas como uma tabela que pode ser escrita em texto-plano.
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
            log.descricao.unwrap_or_default(),
        ]);
    }
    format!("{}\n", table)
}

/// Retorna as últimas operações registradas no log.
///
/// Esta função retorna um Vec contendo estruturas que representam as operações
/// registradas no log, em ordem decrescente de data. A quantidade de registros
/// não excederá o valor imposto através do parâmetro `limite`.
pub fn recupera_log(conexao: &PgConnection, limite: i64) -> Vec<LogDB> {
    use crate::model::schema::logdb::dsl::*;
    logdb
        .order(datahora.desc())
        .limit(limite)
        .load::<LogDB>(conexao)
        .expect("Erro ao recuperar logs")
}
