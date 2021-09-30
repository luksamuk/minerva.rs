// controller/usuarios.rs -- Uma parte de Minerva.rs
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

//! Ferramentas para tráfego de dados entre as rotas de usuários e o banco de
//! dados.
//! 
//! As ferramentas deste módulo realizam o tráfego entre os dados das recebidos
//! através das rotas para gerenciamento de usuários e a tabela `usuario` do
//! banco de dados e relacionadas. 

use super::log::*;
use crate::model::schema::usuario;
use crate::model::schema::usuario::dsl::*;
use crate::model::usuario::{NovoUsuario, Usuario, UsuarioRecv};
use diesel::prelude::*;

/// Lista uma quantidade limitada de usuários cadastrados no sistema.
/// 
/// Retorna um Vec contendo estruturas que representam os dados de um usuário.
/// A quantidade de usuários retornada não deverá exceder a quantidade informada
/// no parâmetro `limite`.
pub fn lista_usuarios(conexao: &PgConnection, limite: i64) -> Vec<Usuario> {
    usuario::table
        .limit(limite)
        .load::<Usuario>(conexao)
        .expect("Erro ao carregar usuários")
}

/// Retorna os dados de um único usuário cadastrado no sistema, através do id.
/// 
/// O valor de retorno é um Option que poderá conter os dados de um usuário de
/// id `usr_id`, caso exista um usuário com este id cadastrado no sistema.
pub fn get_usuario(conexao: &PgConnection, usr_id: i32) -> Option<Usuario> {
    let usr_req = usuario
        .filter(id.eq(&usr_id))
        .load::<Usuario>(conexao)
        .expect("Erro ao carregar usuário");
    usr_req.first().cloned()
}

/// Retorna os dados de um único usuário cadastrado no sistema, através do login.
/// 
/// O valor de retorno é um Option que poderá conter os dados de um usuário cujo
/// login seja o informado através de `usr_login`, caso um usuário com este login
/// exista no sistema.
pub fn encontra_usuario(conexao: &PgConnection, usr_login: String) -> Option<Usuario> {
    let usr_req = usuario
        .filter(login.eq(&usr_login))
        .load::<Usuario>(conexao)
        .expect("Erro ao carregar usuário");
    usr_req.first().cloned()
}

/// Registra um novo usuário no sistema.
/// 
/// Assume-se que os dados informados para cadastro do usuário no sistema
/// sejam válidos. Usuários não poderão ter um login já cadastrado.
/// 
/// Em caso de sucesso, será retornada uma tuple contendo o id e o login do
/// usuário cadastrado, respectivamente. Caso contrário, será retornada uma
/// String contendo uma mensagem de erro.
pub fn registra_usuario(
    conexao: &PgConnection,
    dados: &UsuarioRecv,
) -> Result<(i32, String), String> {
    let novo_usuario = NovoUsuario::from(dados);
    match diesel::insert_into(usuario::table)
        .values(&novo_usuario)
        .get_result::<Usuario>(conexao)
    {
        Ok(usr) => {
            let _ = registra_log(
                conexao,
                String::from("USUARIO"),
                String::from("TO-DO"),
                DBOperacao::Insercao,
                Some(format!("Usuário {} (\"{}\")", usr.id, usr.login)),
            );
            Ok((usr.id, usr.login))
        }
        Err(e) => {
            if let diesel::result::Error::DatabaseError(_, _) = &e {
                Err(format!("{}", e))
            } else {
                Err(String::from(
                    "Erro interno ao cadastrar usuário. \
                                  Contate o suporte para mais informações.",
                ))
            }
        }
    }
}

/// Remove um único usuário do banco de dados, através do seu id.
/// 
/// O usuário a ser removido deverá ter um id igual ao informado através de
/// `usr_id`. A função assume que um usuário com o id informado exista no banco
/// de dados.
pub fn deleta_usuario_por_id(conexao: &PgConnection, usr_id: i32) {
    diesel::delete(usuario.filter(id.eq(&usr_id)))
        .execute(conexao)
        .expect("Erro ao deletar usuário");
    let _ = registra_log(
        conexao,
        String::from("USUARIO"),
        String::from("TO-DO"),
        DBOperacao::Remocao,
        Some(format!("Usuário {}", usr_id)),
    );
}
