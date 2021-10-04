// model/usuario.rs -- Uma parte de Minerva.rs
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

//! Utilitários de modelagem de usuário do sistema para banco de dados e
//! regras de negócio.
//!
//! Este módulo define estruturas para o tráfego de dados de usuários do sistema
//! entre as partes respectivas do mesmo. Estes usuários serão as entidades que
//! efetuam operações no sistema, desde que autorizados via login.
//!
//! Para informações relacionadas a login e autenticação, veja o módulo
//! [`login`][`super::login`] e o módulo [`auth`][`crate::auth`].

use crate::bo::usuarios;
use crate::model::schema::usuario;
use serde::{Deserialize, Serialize};

/// Representa um usuário do sistema cadastrado no banco de dados.
///
/// Os dados do usuário também poderão ser serializados para JSON e retornados
/// através de alguma requisição de dados de usuário. Porém, o hash gerado a
/// partir da senha, no momento do cadastro, jamais será retornado, permanecendo
/// invisível e interno para a aplicação.
#[derive(Queryable, Serialize, Clone)]
pub struct Usuario {
    /// Id de cadastro do usuário no banco de dados.
    pub id: i32,
    /// Login do usuário no banco de dados. Deverá ser único para o mesmo. Não
    /// deve possuir espaços ao início e nem ao final.
    pub login: String,
    /// Nome do usuário. Não deve possuir espaços ao início e nem ao final.
    pub nome: String,
    /// E-mail do usuário. Não precisa ser informado. Não deve possuir espaços
    /// ao início e nem ao final.
    pub email: Option<String>,
    /// Hash da senha do usuário, gerada no momento do cadastro. Este hash
    /// possui os elementos descritos na
    /// [função de geração de hash][`crate::bo::usuarios::gera_hash_senha`].
    #[serde(skip_serializing)]
    pub senha_hash: Vec<u8>,
}

/// Representa os dados de inserção de um novo usuário no banco de dados.
#[derive(Insertable)]
#[table_name = "usuario"]
pub struct NovoUsuario {
    /// Login do usuário no banco de dados.
    /// Veja [`Usuario::login`].
    pub login: String,
    /// Nome do usuário.
    /// Veja [`Usuario::nome`].
    pub nome: String,
    /// E-mail do usuario.
    /// Veja [`Usuario::email`].
    pub email: Option<String>,
    /// Hash da senha do usuário.
    /// Veja [`Usuario::senha_hash`].
    pub senha_hash: Vec<u8>,
}

/// Representa os dados de criação de um novo usuário, recebidos via requisição
/// de tipo POST.
///
/// Os dados recebidos via requisição terão um tempo de vida iguais à estrutura,
/// especialmente por questões de segurança.
///
/// O cadastro de um usuário deverá ser recebido como mostrado a seguir, através
/// de uma estrutura em JSON:
///
/// ```json
/// {
///   "login": "fulano",
///   "nome": "Fulano de Tal",
///   "email": "fulano@exemplo.com",
///   "senha": "senhadofulano"
/// }
/// ```
///
/// Veja que o e-mail do usuário sendo cadastrado é opcional, podendo ser
/// omitido ou definido como `null`.
#[derive(Deserialize, Clone)]
pub struct UsuarioRecv<'r> {
    /// Login do usuário a ser cadastrado.
    /// Veja [`Usuario::login`].
    pub login: &'r str,
    /// Nome do usuário a ser cadastrado.
    /// Veja [`Usuario::nome`].
    pub nome: &'r str,
    /// E-mail do usuário a ser cadastrado. Opcional.
    /// Veja [`Usuario::email`].
    pub email: Option<&'r str>,
    /// Senha do usuário a ser cadastrado, em texto-plano. Será transformado
    /// em hash para armazenamento.
    /// Veja [`Usuario::senha_hash`].
    #[serde(skip_serializing)]
    pub senha: &'r str,
}

impl<'r> From<&UsuarioRecv<'r>> for NovoUsuario {
    /// Realiza conversão e tratamento dos dados de um novo usuário, quando
    /// recebidos via requisição POST, para dados prontos para serem inseridos
    /// no banco de dados.
    ///
    /// Esta conversão, em especial, removerá espaços em branco ao início e ao
    /// final de todos os campos, e gerará o
    /// [hash da senha][`crate::bo::usuarios::gera_hash_senha`] informada como
    /// texto.
    fn from(usr: &UsuarioRecv) -> Self {
        NovoUsuario {
            login: usr.login.trim().to_string(),
            nome: usr.nome.trim().to_string(),
            email: usr.email.as_ref().map(|email| email.trim().to_string()),
            senha_hash: usuarios::gera_hash_senha(usr.senha.trim().as_bytes())
                .0
                .to_vec(),
        }
    }
}
