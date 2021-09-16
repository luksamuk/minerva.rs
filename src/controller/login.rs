// controller/login.rs -- Uma parte de Minerva.rs
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

use super::usuarios;
use crate::auth::jwt;
use crate::db::redis::RedisConnection;
use crate::model::login::{LoginData, LoginResponse};
use crate::routes::respostas::Resposta;
use diesel::PgConnection;
use r2d2_redis::redis::Commands;

pub fn loga_usuario(
    conexao: &PgConnection,
    redis: &mut RedisConnection,
    dados: &LoginData,
) -> Resposta {
    // 1. Verifica se o usuário existe.
    let usuario = match usuarios::encontra_usuario(conexao, dados.login.to_owned()) {
        Some(usuario) => usuario,
        None => {
            return Resposta::NaoEncontrado(String::from(
                "{ \"mensagem\": \"Usuário não encontrado\" }",
            ))
        }
    };

    // 2. Testa a senha do usuário.
    if !usuario.verifica_senha(dados.senha) {
        return Resposta::NaoAutorizado(String::from("{ \"mensagem\": \"Senha incorreta\" }"));
    }

    // 3. Gera token JWT.
    let token = match jwt::cria_jwt(&dados.login) {
        Err(erro) => {
            return Resposta::ErroInterno(format!(
                "{{ \"mensagem\": \"Erro ao gerar token JWT: {}\" }}",
                erro
            ))
        }
        Ok(tok) => tok,
    };

    // 4. Salva token no Redis com expiração de 5m30s
    match redis.set_ex::<&str, &str, String>(&token, dados.login, jwt::JWT_MAX_SECONDS) {
        Err(_) => {
            return Resposta::ErroInterno(String::from(
                "{ \"mensagem\": \"Erro ao registrar token JWT\" }",
            ))
        }
        _ => {}
    }

    // 5. Retorna o token.
    Resposta::Ok(
        serde_json::to_string(&LoginResponse {
            id: usuario.id,
            login: dados.login.to_owned(),
            token: token,
        })
        .unwrap(),
    )
}
