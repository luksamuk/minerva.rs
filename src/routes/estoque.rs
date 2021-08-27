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
use rocket::State;
use crate::db::ConexaoPool;
use crate::model::estoque::MovEstoque;
use super::respostas::Resposta;

pub fn constroi_rotas() -> Vec<Route> {
    routes![movimenta_estoque]
}

#[post("/", data = "<dados>")]
fn movimenta_estoque(
    pool: &State<ConexaoPool>,
    dados: Json<MovEstoque>
) -> Resposta {
    let conexao = pool.get().unwrap();
    match produtos::get_produto(&conexao, dados.produto_id) {
        None => Resposta::NaoEncontrado(
            String::from("{ \"mensagem\": \"Produto não encontrado\" }")),
        Some(p) =>
            produtos::muda_estoque(&conexao, &p, dados.quantidade.clone()),
    }
}
