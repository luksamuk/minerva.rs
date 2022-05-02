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

//! Ferramentas relacionadas ao gerenciamento de sessão do usuário.
//!
//! Este módulo descreve funções capazes de gerenciar operações como _login_ e
//! _logout_ para cada usuário, realizando portanto a ponte entre as rotas para
//! este propósito e os processos de autenticação do sistema.

use super::usuarios;
use crate::bo;
use crate::bo::auth::jwt;
use crate::bo::db::ConexaoPool;
//use crate::bo::redis::RedisConnection;
use crate::bo::redis::RedisPool;
use crate::model::login::{LoginData, LoginResponse};
use crate::routes::respostas::Resposta;
use bb8_redis::redis::AsyncCommands;
use serde_json::json;

/// Realiza login para um usuário.
///
/// A função espera por uma referência aos dados de login, sendo estes o nome
/// do usuário e sua senha, e também requisita conexões com o banco de dados e
/// com o serviço Redis.
///
/// A função também verifica pela existência do usuário, realiza conferência de
/// senha, e por fim gera um token JWT com expiração gerenciada através do
/// Redis.
///
/// Um retorno com sucesso conterá o id, o login e o token do usuário logado.
/// Falhas ao encontrar o usuário retornarão um erro 404. Falhas na autenticação
/// retornarão um erro 401. Caso haja alguma falha ao gerar o token JWT ou ao
/// registrá-lo no Redis, será retornado um erro 500.
pub async fn loga_usuario(
    conexao_pool: &ConexaoPool,
    // redis: &mut RedisConnection<'_>,
    redis_pool: &RedisPool,
    dados: &LoginData,
) -> Resposta {
    let conexao = conexao_pool.get().await.unwrap();
    // 1. Verifica se o usuário existe.
    let usuario = match usuarios::encontra_usuario(&conexao, &dados.login) {
        Some(usuario) => usuario,
        None => {
            return Resposta::NaoEncontrado(
                json!({
                    "mensagem": "Usuário não encontrado"
                })
                .to_string(),
            )
        }
    };

    // 2. Testa a senha do usuário.
    if !bo::usuarios::verifica_senha(&dados.senha, &usuario.senha_hash) {
        return Resposta::NaoAutorizado(
            json!({
                "mensagem": "Senha incorreta"
            })
            .to_string(),
        );
    }

    // 3. Gera token JWT.
    let token = match jwt::cria_jwt(&dados.login) {
        Ok(tok) => tok,
        Err(erro) => {
            return Resposta::ErroInterno(
                json!({ "mensagem": format!("Erro ao gerar token JWT: {}", erro) }).to_string(),
            )
        }
    };

    // 4. Salva token no Redis com expiração de 5m30s
    let mut redis = redis_pool.get().await.unwrap();
    if redis
        .set_ex::<&str, &str, String>(&token, &dados.login, jwt::JWT_SESSION_EXPIRATION)
        .await
        .is_err()
    {
        return Resposta::ErroInterno(
            json!({
                "mensagem": "Erro ao registrar token JWT"
            })
            .to_string(),
        );
    }

    // 5. Envia mensagem assíncrona via Twilio
    // use crate::bo::whatsapp;
    // whatsapp::envia_arquivo_whatsapp_sandbox(
    //     format!("O usuário {} acabou de fazer login (id = {})",
    //             dados.login,
    //             usuario.id),
    //     if dados.login == "admin" {
    //         "documento.pdf"
    //     } else if dados.login == "sanic" {
    //         "sanic.png"
    //     } else {
    //         "sonica.png"
    //     },
    // );

    // 5. Retorna o token.
    Resposta::Ok(
        serde_json::to_string(&LoginResponse {
            id: usuario.id,
            login: dados.login.to_owned(),
            token,
        })
        .unwrap(),
    )
}
