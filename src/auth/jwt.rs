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

use jsonwebtoken::{ Algorithm, Header, EncodingKey, DecodingKey, Validation };
use serde::{ Serialize, Deserialize };

// 5m30s por padrão
pub const JWT_MAX_SECONDS: usize = 330;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

// TODO: Token gerado via hash md5. Ex: "echo -n 'minhachave' | md5sum"
// Seria mais interessante se gerássemos isso aleatoriamente para cada
// usuário e salvássemos seus respectivos hashes no banco de dados de
// tempos em tempos, mas fica pra outra situação, pois dessa forma
// precisaríamos acessar o BD, e portanto precisaríamos tornar o
// Request Guard baseado em Request-Local State (ver documentação
// do trait FromRequest do Rocket). Por enquanto, vamos deixar esse
// hash fixo mesmo, pois essa implementação é para aprendizado.
// Se isso aparecer algum dia em produção, TÁ ERRADO, MUDE AGORA!
const JWT_SECRET: &[u8] = b"89d302d91e93b2b13f8284eb389ef15d";


// ATENÇÃO: A invalidação de JWT deve ser realizada em conjunto com
// armazenamento no Redis. Não vamos confiar inteiramente no JWT,
// pois ele poderia ser adulterado.
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
        .map_err(|_| "Erro ao criar token JWT".to_string())
}

pub fn decodifica_jwt(jwt: &str) -> Result<JwtClaims, String> {
    let decodificado = jsonwebtoken::decode::<JwtClaims>(
        jwt,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512)
    ).map_err(|_| "Erro ao decodificar JWT".to_owned())?;

    Ok(decodificado.claims)
}
