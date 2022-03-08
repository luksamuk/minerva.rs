// model/login.rs -- Uma parte de Minerva.rs
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

//! Utilitários de modelagem de realização de login de um usuário.
//!
//! Os utilitários deste módulo compreendem estruturas utilizadas para realizar
//! o login de um usuário no sistema, sendo especialmente utilizadas para
//! tráfego de dados de login no mesmo.
//!
//! Para informações sobre dados de um usuário, veja [`usuario`][`super::usuario`].

use serde::{Deserialize, Serialize};

/// Representa dados de login recebidos através de uma requisição `POST` na rota
/// de login. Os dados textuais desta requisição deverão ser referências diretas
/// aos dados recebidos, para evitar cópias, já que estes dados são sensíveis.
///
/// É essencial que os dados sejam repassados através de uma conexão encriptada
/// com SSL, para maior segurança.
///
/// Os dados de login são esperados no seguinte formato, em JSON:
///
/// ```json
/// {
///   "login": "admin",
///   "senha": "admin"
/// }
/// ```
#[derive(Deserialize)]
pub struct LoginData<'r> {
    /// Login do usuário tentando realizar login.
    pub login: &'r str,
    /// Senha do usuário tentando realizar login, repassada em texto plano.
    pub senha: &'r str,
}

/// Representa dados de login a serem respondidos, em caso de um login
/// bem-sucedido.
///
/// Estes dados compreendem alguns dados do usuário logado e um JWT como token
/// de acesso, que será retornado para uso como Bearer Token em requisições
/// subsequentes do usuário.
///
/// A resposta será em formato JSON, como mostrado no exemplo:
///
/// ```json
/// {
///   "id": 1,
///   "login": "admin",
///   "token": "..."
/// }
/// ```
///
/// Note que o campo `token` será um JSON Web Token apropriadamente gerado, no
/// lugar das reticências. Para maiores informações sobre a geração deste token,
/// veja o módulo [`auth`][`crate::auth`].
#[derive(Serialize)]
pub struct LoginResponse {
    /// Id do usuário logado no banco de dados.
    pub id: i32,
    /// Login do usuário logado no sistema.
    pub login: String,
    /// JSON Web Token para uso como Bearer Token de autenticação em
    /// requisições subsequentes.
    pub token: String,
}
