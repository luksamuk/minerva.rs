// auth/mod.rs -- Uma parte de Minerva.rs
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

//! Utilitários relacionados a autenticação do usuário.
//!
//! Este módulo possui utilitários para autenticação através de Bearer Token ao
//! realizar requisições na API. O Bearer Token deverá ser incluído no cabeçalho
//! das requisições que necessitarem de login do usuário.
//!
//! # Obtendo um Bearer Token
//! Suponha que o usuário realizou uma requisição de login na rota `/login`.
//! Esta requisição será do tipo POST e terá um formato JSON contendo o login e
//! a senha do mesmo, como no exemplo a seguir, que pode ser feito utilizando a
//! ferramenta de linha de comando `curl`:
//!
//! ```bash
//!  curl --request POST \
//!   --url https://url-da-api/login \
//!   --header 'Content-Type: application/json' \
//!   --data '{
//!     "login": "admin",
//!     "senha": "admin"
//! }'
//! ```
//! Em caso de sucesso, esta rota devolverá uma resposta em JSON contendo
//! algumas informações sobre o usuário, o que também incluirá um token, como no
//! exemplo a seguir (substituído por reticências):
//!
//! ```json
//! {
//!   "id": 1,
//!   "login": "admin",
//!   "token": "..."
//! }
//! ```
//! O token retornado é um [JSON Web Token], que deverá ser incluído nas
//! requisições que exigem login.
//!
//! [JSON Web Token]: https://jwt.io
//!
//! # Uso do Bearer Token
//! Para utilizar o Bearer Token, é necessário informá-lo no cabeçalho das
//! requisições sob o item `Authorization`. Normalmente o Bearer Token é
//! informado utilizando a palavra `Bearer`, seguida de um espaço e o conteúdo
//! do token, como retornado na requisição de exemplo anterior.
//!
//! O exemplo a seguir demonstra o uso do Bearer Token no cabeçalho de uma
//! requisição de tipo GET na rota `/usuarios`, que lista os usuários
//! cadastrados no sistema até certo limite. O conteúdo do token foi substituído
//! por reticências para exemplificação.
//!
//! ```bash
//! curl --request GET \
//!   --url https://url-da-api/usuarios \
//!   --header 'Authorization: Bearer ...'
//! ```
//!
//! # Informações de segurança
//! Existem ressalvas relacionadas à segurança de um JWT. Uma das falhas
//! notáveis está relacionada ao fato de um JWT ser _stateless_, e portanto
//! o tempo de sessão geralmente envolve apenas o tempo de expiração do JWT.
//! Isso dificulta a possibilidade de _logout_ e abre margem para operações
//! fraudulentas envolvendo tokens obtidos por terceiros, sem o consentimento
//! do usuário original.
//!
//! Para mitigar parcialmente estes problemas, os JWTs aqui emitidos têm um
//! tempo de vida padrão de cinco minutos e trinta segundos (5m30s, ou 330
//! segundos). Além disso, todos os JWTs emitidos são armazenados
//! temporariamente como chaves no serviço Redis, e associados ao nome do
//! usuário que os emitiu.
//!
//! Os tokens armazenados no Redis têm, igualmente, um tempo de expiração
//! de 5m30s a partir de sua inserção. Isso significa que a expiração do JWT
//! é gerenciada via Redis, além da própria expiração inerente ao mesmo;
//! além disso, caso o token seja adulterado de alguma forma, por não estar
//! armazenado no serviço, o mesmo será recusado como inválido.
//!
//! Através do uso desse mesmo recurso, é possível invalidar prematuramente
//! um token, o que possibilita a implementação de um _logout_.[^todo]
//!
//! [^todo]: O processo de _logout_ ainda não foi implementado.

pub mod jwt;

use crate::db::redis::RedisConnection;
use crate::db::RedisPool;
use r2d2_redis::redis::Commands;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

/// Representa a chave de autenticação do usuário atualmente acessando a rota.
///
/// A estrutura carrega chave (JWT) de autenticação do usuário e implementa um
/// [_request guard_][`rocket::request::FromRequest`], o que significa que ela
/// poderá ser incluída como parâmetro em uma rota para que a mesma requeira o
/// uso de um Bearer Token de autenticação.
///
/// O exemplo a seguir demonstra o uso de uma rota que requer autenticação.
///
/// ```rust
/// #[get("/")]
/// fn index(auth: AuthKey<'_>) -> String {
///     format!("Olá mundo! Seu JWT: {}", auth.0)
/// }
/// ```
pub struct AuthKey<'r>(&'r str);

/// Representa um erro de autenticação.
#[derive(Debug)]
pub enum AuthError {
    /// Chave (Bearer Token) não fornecido na requisição.
    KeyMissing,
    /// Usuário não está logado no sistema.
    NotLoggedIn,
}

/// Extrai o payload de um Bearer Token.
/// Basicamente apenas extrai o prefixo do parâmetro do cabeçalho.
fn extrai_payload(token_completo: &str) -> String {
    if token_completo.starts_with("Bearer ") {
        token_completo.trim_start_matches("Bearer ").to_owned()
    } else {
        token_completo.to_owned()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthKey<'r> {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let redis_pool = req.rocket().state::<RedisPool>().unwrap();
        let mut redis = redis_pool.get().unwrap();

        fn is_valid(chave: &str, redis: &mut RedisConnection) -> bool {
            let payload = extrai_payload(chave);
            let claims = jwt::decodifica_jwt(&payload);
            if claims.is_err() {
                return false;
            }

            let claims = claims.unwrap();

            let usuario_associado = redis.get::<String, String>(payload.clone());
            match usuario_associado {
                Ok(login) => {
                    if login == claims.sub {
                        let _ =
                            redis.expire::<String, String>(payload, jwt::JWT_SESSION_EXPIRATION);
                        return true;
                    }
                    false
                }
                Err(_) => false,
            }
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, AuthError::KeyMissing)),
            Some(chave) if is_valid(chave, &mut redis) => Outcome::Success(AuthKey(chave)),
            Some(_) => Outcome::Failure((Status::Unauthorized, AuthError::NotLoggedIn)),
        }
    }
}
