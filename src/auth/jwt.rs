// auth/jwt.rs -- Uma parte de Minerva.rs
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

//! Emissão e validação de JSON Web Tokens
//!
//! Este módulo possui estruturas e rotinas para gerenciar a emissão e a
//! validação de JSON Web Tokens (JWTs).

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Número padrão de segundos de validade de uma sessão usanod JSON Web Token.
///
/// O sistema determina um tempo de expiração para JSON Web Tokens, de forma
/// independente da validade dos mesmos. Este tempo de expiração de sessão é
/// redefinido a cada requisição autorizada.
pub const JWT_SESSION_EXPIRATION: usize = 330;

/// Número padrão de segundos de validade de um JSON Web Token emitido.
///
/// JSON Web Tokens possuem, por padrão, um tempo de vida de sete dias.
pub const JWT_MAX_SECONDS: usize = 3604800;

/// Representa os claims do payload de um JSON Web Token.
///
/// JWTs possuem informações inerentes a si, que podem ser opcionalmente
/// informadas em seu payload.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JwtClaims {
    /// Sujeito da informação. Carrega o login do usuário que emitiu o token.
    pub sub: String,
    /// Data exata de expiração do token, em timestamp Unix.
    pub exp: usize,
    /// Data exata de emissão do token, em timestamp Unix.
    pub iat: usize,
}

/// Token secreto para emissão de JWT, gerado via hash MD5. Pode ser gerado a
/// partir de uma chave com o seguinte comando:
///
/// ```bash
/// echo -n 'minhachave' | md5sum
/// ```
///
/// Seria mais interessante se gerássemos isso aleatoriamente para cada usuário
/// e salvássemos seus respectivos hashes no banco de dados de tempos em tempos,
/// mas fica para outra situação, pois dessa forma precisaríamos acessar o banco
/// de dados, e portanto precisáriamos tornar o request guard
/// [`super::AuthKey`] um guard baseado em request-local state[^ver-docs].
///
/// Por enquanto, vamos deixar esse hash fixo mesmo, pois essa implementação foi
/// criada para fins didáticos.
///
/// Mas se isso aparecer em produção e você esbarrar nela, *NÃO FUJA. TÁ ERRADO.
/// MUDE ELA AGORA MESMO.*
///
/// [^ver-docs]: Ver documentação do trait [`rocket::request::FromRequest`].
const JWT_SECRET: &[u8] = b"89d302d91e93b2b13f8284eb389ef15d";

/// Cria um JSON Web Token para um login de usuário em específico.
///
/// Gera um JWT válido para um usuário com o login informado. O JWT gerado terá
/// em seu payload as informações de [`JwtClaims`], podendo ser posteriormente
/// decodificado para essa estrutura.
///
/// É importante notar que, por mais que um JWT possua um tempo de expiração e
/// um método de verificação, estas ferramentas não serão tratadas como
/// confiáveis no âmbito da API, algo mitigado através do uso do serviço
/// Redis[^ver-super].
///
/// [^ver-super]: Para mais informações, ver a documentação exposta em
/// [`crate::auth`].
///
/// # Resultado
/// A função retornará o JWT em formato de texto, caso a operação seja
/// bem-sucedida, ou uma mensagem de erro caso não seja possível criar o JWT.
///
/// # Panics
/// A função entrará em pânico caso ocorra um erro ao gerar o timestamp correto
/// de expiração do token.
pub fn cria_jwt(login: &str) -> Result<String, String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(JWT_MAX_SECONDS as i64))
        .expect("Timestamp inválido")
        .timestamp();

    let claims = JwtClaims {
        sub: login.to_owned(),
        exp: expiration as usize,
        iat: chrono::Utc::now().timestamp() as usize,
    };

    let cabecalho = Header::new(Algorithm::HS512);
    jsonwebtoken::encode(&cabecalho, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| "Erro ao criar token".to_string())
}

/// Decodifica um JSON Web Token, caso seja válido.
///
/// Decodifica o payload de um JWT válido, retornando uma estrutura
/// [`JwtClaims`] populada com suas informações inerentes. O token será
/// considerado válido se sua assinatura for válida, de acordo com a
/// [decodificação padrão][`jsonwebtoken::decode`] prevista pela biblioteca
/// utilizada.
///
/// É importante notar que, por mais que um JWT possua um tempo de expiração e
/// um método de verificação, estas ferramentas não serão tratadas como
/// confiáveis no âmbito da API, algo mitigado através do uso do serviço
/// Redis[^ver-super].
///
/// [^ver-super]: Para mais informações, ver a documentação exposta em
/// [`crate::auth`].
///
/// # Resultado
/// A função retornará a estrutura de claims devidamente populada, caso o JWT
/// seja válido, ou uma mensagem de erro caso o mesmo seja inválido ou não puder
/// ser decodificado.
pub fn decodifica_jwt(jwt: &str) -> Result<JwtClaims, String> {
    let decodificado = jsonwebtoken::decode::<JwtClaims>(
        jwt,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| "Erro ao decodificar JWT".to_owned())?;

    Ok(decodificado.claims)
}
