// db/postgres.rs -- Uma parte de Minerva.rs
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

//! Estruturas relacionadas à conexão com o banco de dados PostgreSQL.
//! 
//! As estruturas aqui descritas dizem respeito à conexão com o banco de dados,
//! bem como com as estruturas de pool de conexões e de garantia mínima de
//! usuário inicial do sistema.

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

/// Representa um pool de conexões com o RDBMS PostgreSQL.
pub type ConexaoPool = Pool<ConnectionManager<PgConnection>>;

/// Cria um pool de conexões com o RDBMS PostgreSQL.
/// Deve ser chamada uma vez ao início da aplicação.
///
/// O URL para o serviço do PostgreSQL deve ser definido através da variável de
/// ambiente `DATABASE_URL`, que também pode ser definida em um arquivo `.env`
/// no diretório em que a aplicação for iniciada, com a seguinte formatação:
///
/// `DATABASE_URL=postgres://usuario:senha@localhost/nomedobanco`
///
/// # Panics
///
/// A função entrará em pânico se a variável `DATABASE_URL` não for definida no
/// ambiente, e se algum erro anormal ocorrer ao criar a pool de conexões.
///
/// # Exemplo
/// O exemplo a seguir cria um pool, recupera uma conexao com o PostgreSQL e
/// carrega um vetor de um máximo de 100 clientes a partir do banco de dados.
///
/// ```
/// use minerva::model::schema::cliente;
/// use minerva::model::cliente::Cliente;
/// usr minerva::db::postgres::*;
///
/// let pool = cria_pool_conexoes();
/// let conexao = pool.get().unwrap();
/// let vec_clientes = cliente::table.limit(100)
///     .load::<Cliente>(&conexao)
///     .expect("Erro ao consultar a tabela de clientes");
/// ```
/// A conexão será devolvida ao pool ao sair do escopo atual.
pub fn cria_pool_conexoes() -> ConexaoPool {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Necessário definir o URL do BD em DATABASE_URL");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    Pool::builder()
        .build(manager)
        .expect("Falha ao criar pool de conexões.")
}

/// Garante a existência de um usuário no banco de dados. Caso nenhum usuário
/// esteja cadastrado, cria um usuário padrão.
///
/// Esta função foi pensada para cobrir situações em que os usuários pudessem
/// ser apagados do banco de dados, ou para as situações em que nenhum usuário
/// tiver sido ainda cadastrado.
///
/// Neste caso, a função criará um usuário com as seguintes especificações:
///
/// - Nome: Admin
/// - Login: `admin`
/// - Senha: `admin`
///
/// O usuário será inserido no banco de dados com a senha tendo sido
/// apropriadamente encriptada, por mais que a senha seja um dado público,
/// já que a encriptação da senha é uma regra de negócio da aplicação.
///
/// # Panics
/// A função entrará em pânico caso a inserção do usuário no banco de dados
/// falhe.
pub fn garante_usuario_inicial(pool: &ConexaoPool) {
    use crate::controller::usuarios;
    use crate::model::schema::usuario;
    use crate::model::usuario::{NovoUsuario, UsuarioRecv};
    use diesel::prelude::*;

    let conexao = pool.get().unwrap();
    if usuarios::lista_usuarios(&conexao, 1).is_empty() {
        let novo_admin = NovoUsuario::from(&UsuarioRecv {
            login: String::from("admin"),
            nome: String::from("Admin"),
            email: None,
            senha: String::from("admin"),
        });
        let _ = diesel::insert_into(usuario::table)
            .values(&novo_admin)
            .execute(&conexao)
            .expect("Erro ao cadastrar usuário \"admin\"");
    }
}
