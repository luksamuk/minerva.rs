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

use rocket::response::Responder;

#[derive(Responder)]
pub enum Resposta {
    #[response(status = 200, content_type = "json")]
    Ok(String),
    #[response(status = 404, content_type = "json")]
    NaoEncontrado(String),
    #[response(status = 418, content_type = "text")]
    Chaleira(&'static str),
    #[response(status = 422, content_type = "json")]
    ErroSemantico(String),
}
