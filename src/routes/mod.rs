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
pub mod log;

use respostas::Resposta;

#[get("/")]
pub fn index() -> Resposta {
    use comfy_table::{ Table, presets::ASCII_BORDERS_ONLY_CONDENSED };
    let mut table = Table::new();
    table.load_preset(ASCII_BORDERS_ONLY_CONDENSED)
        .set_header(vec!["Requisição", "Rota", "Descrição"]);
    
    table.add_row(vec!["GET",    "/",              "Lista de rotas",              ]);

    table.add_row(vec!["GET",    "/produtos",      "Lista de produtos",           ]);
    table.add_row(vec!["POST",   "/produtos",      "Cadastra um produto",         ]);
    table.add_row(vec!["GET",    "/produtos/<id>", "Mostra um produto",           ]);
    table.add_row(vec!["DELETE", "/produtos/<id>", "Remove um produto",           ]);
    table.add_row(vec!["DELETE", "/produtos/all",  "Remove todos os produtos",    ]);

    table.add_row(vec!["POST",   "/estoque",       "Faz movimentação de estoque", ]);

    table.add_row(vec!["GET",    "/clientes",      "Lista de clientes",           ]);
    table.add_row(vec!["POST",   "/clientes",      "Cadastra um cliente",         ]);
    table.add_row(vec!["GET",    "/clientes/<id>", "Mostra um cliente",           ]);
    table.add_row(vec!["DELETE", "/clientes/<id>", "Deleta um cliente",           ]);
    table.add_row(vec!["DELETE", "/clientes/all",  "Deleta todos os clientes",    ]);

    table.add_row(vec!["GET",    "/log/txt",       "Tabela de log (texto plano)", ]);

    Resposta::Chaleira(format!("Lista de rotas\n{}\n", table))
}
