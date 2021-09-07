// routes/estoque.rs -- Uma parte de Minerva.rs
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

use rocket::Route;
use rocket::serde::json::Json;
use crate::controller::produtos;
use crate::controller::estoque;
use rocket::State;
use crate::db::ConexaoPool;
use crate::model::estoque::{ Estoque, MovEstoqueRecv };
use super::respostas::Resposta;

// Rotas planejadas:
// GET    /estoque => Lista o estoque de todos os produtos -- talvez?
// [OK] POST   /estoque => Realiza início de estoque para um produto em específico

// GET    /estoque/id => Lista o estoque de um produto em específico
// DELETE /estoque/id => Deleta estoque de um produto

// GET    /estoque/mov => Mostra movimentações de estoque mais reentes
// [OK] POST   /estoque/mov => Realiza uma movimentação de estoque (entrada ou saída).
//                        Não aceitar movimentações com quantidade ou preço
//                        zerados!

// GET    /estoque/mov/entradas => Entradas mais recentes
// GET    /estoque/mov/saidas => Saídas mais recentes

// GET    /estoque/mov/entradas/txt => Entradas mais recentes (texto)
// GET    /estoque/mov/saidas/txt => Saídas mais recentes (texto)

// Necessário também garantir que essas movimentações de MOV_ESTOQUE
// alterem ESTOQUE de acordo.

pub fn constroi_rotas() -> Vec<Route> {
    routes![inicia_estoque, movimenta_estoque]
}

#[post("/", data = "<dados>")]
fn inicia_estoque(pool: &State<ConexaoPool>, dados: Json<Estoque>) -> Resposta {
    let conexao = pool.get().unwrap();
    estoque::inicia_estoque(&conexao, dados.clone())
}

#[post("/mov", data = "<dados>")]
fn movimenta_estoque(
    pool: &State<ConexaoPool>,
    dados: Json<MovEstoqueRecv>
) -> Resposta {
    let conexao = pool.get().unwrap();
    estoque::movimenta_estoque(&conexao, dados.clone())
}
