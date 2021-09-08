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

pub mod jwt;

use rocket::http::Status;
use rocket::request::{ Outcome, Request, FromRequest };
use crate::db::RedisPool;
use crate::db::redis::RedisConnection;
use r2d2_redis::redis::Commands;

pub struct AuthKey<'r>(&'r str);

#[derive(Debug)]
pub enum AuthError {
    KeyMissing,
    NotLoggedIn,
}

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

            let usuario_associado = redis.get::<String, String>(payload);
            match usuario_associado {
                Ok(login) => login == claims.sub,
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
