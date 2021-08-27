// routes/mod.rs -- Uma parte de Minerva.rs
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

pub mod respostas;
pub mod clientes;
pub mod produtos;
pub mod estoque;

use respostas::Resposta;

#[get("/")]
pub fn index() -> Resposta {
    Resposta::Chaleira(
        "Lista de rotas                \n\
         ==============================\n\
         GET    /                      \n\
         ------------------------------\n\
         GET    /produtos              \n\
         POST   /produtos              \n\
         GET    /produtos/<id>         \n\
         DELETE /produtos/<id>         \n\
         DELETE /produtos/all          \n\
         ------------------------------\n\
         POST   /estoque               \n\
         ------------------------------\n\
         GET    /clientes              \n\
         POST   /clientes              \n\
         GET    /clientes/<id>         \n\
         DELETE /clientes/<id>         \n\
         DELETE /clientes/all          \n\
         ")
}
