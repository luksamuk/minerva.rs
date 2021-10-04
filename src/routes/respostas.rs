// routes/respostas.rs -- Uma parte de Minerva.rs
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

//! Definições de respostas HTTP para situações comuns do sistema.

use rocket::response::Responder;

/// Representa uma resposta via requisição web, vinda da aplicação.
/// A resposta envolve um código de retorno HTTP e dados associados, que poderão
/// ser JSON ou texto plano.
#[derive(Responder)]
pub enum Resposta {
    /// Resposta de sucesso 200 com retorno em JSON.
    #[response(status = 200, content_type = "json")]
    Ok(String),
    /// Resposta de sucesso 200 com retorno em texto plano.
    #[response(status = 200, content_type = "text")]
    OkTexto(String),
    /// Resposta de erro 401 com retorno em JSON, para situações
    /// em que o usuário não estiver autorizado a acessar o recurso.
    #[response(status = 401, content_type = "json")]
    NaoAutorizado(String),
    /// Resposta de erro 404 com retorno em JSON, para situações
    /// em que o recurso acessado não for encontrado.
    #[response(status = 404, content_type = "json")]
    NaoEncontrado(String),
    /// Resposta de erro 418 com retorno em texto plano, caso o
    /// usuário queira muito tomar um cafezinho.
    #[response(status = 418, content_type = "text")]
    Chaleira(String),
    /// Resposta de erro 422 com retorno em JSON, para situações
    /// em que os dados fornecidos pelo usuário estejam semanticamente
    /// incorretos, ou com formato inválido, ou algo similar.
    #[response(status = 422, content_type = "json")]
    ErroSemantico(String),
    /// Resposta de erro 500 com retorno em JSON, para situações
    /// em que a aplicação encontrar um erro e não conseguir se
    /// recuperar do mesmo.
    #[response(status = 500, content_type = "json")]
    ErroInterno(String),
}
