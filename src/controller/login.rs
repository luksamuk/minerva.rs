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

use crate::bo;
use super::usuarios;
use crate::auth::jwt;
use crate::db::redis::RedisConnection;
use crate::model::login::{LoginData, LoginResponse};
use crate::routes::respostas::Resposta;
use diesel::PgConnection;
use r2d2_redis::redis::Commands;

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
pub fn loga_usuario(
    conexao: &PgConnection,
    redis: &mut RedisConnection,
    dados: &LoginData,
) -> Resposta {
    // 1. Verifica se o usuário existe.
    let usuario = match usuarios::encontra_usuario(conexao, dados.login) {
        Some(usuario) => usuario,
        None => {
            return Resposta::NaoEncontrado(String::from(
                "{ \"mensagem\": \"Usuário não encontrado\" }",
            ))
        }
    };

    // 2. Testa a senha do usuário.
    if !bo::usuarios::verifica_senha(dados.senha, &usuario.senha_hash) {
        return Resposta::NaoAutorizado(String::from("{ \"mensagem\": \"Senha incorreta\" }"));
    }

    // 3. Gera token JWT.
    let token = match jwt::cria_jwt(dados.login) {
        Err(erro) => {
            return Resposta::ErroInterno(format!(
                "{{ \"mensagem\": \"Erro ao gerar token JWT: {}\" }}",
                erro
            ))
        }
        Ok(tok) => tok,
    };

    // 4. Salva token no Redis com expiração de 5m30s
    if redis
        .set_ex::<&str, &str, String>(&token, dados.login, jwt::JWT_SESSION_EXPIRATION)
        .is_err()
    {
        return Resposta::ErroInterno(String::from(
            "{ \"mensagem\": \"Erro ao registrar token JWT\" }",
        ));
    }

    // 5. Envia mensagem assíncrona via Twilio
    use crate::bo::twilio;
    twilio::envia_mensagem_whatsapp_sandbox(
        format!("O usuário {} acabou de fazer login (id = {})\n\
                 Token de autenticação:\n{}",
                dados.login, usuario.id, token)
    );

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
