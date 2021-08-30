// bin/cria_cliente.rs -- Uma parte de Minerva.rs
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

extern crate minerva;
extern crate diesel;

use minerva::model::schema::{ cliente, endereco };
use minerva::model::cliente::{ NovoCliente, Cliente };
use minerva::model::endereco::{ NovoEndereco, Endereco };
use diesel::prelude::*;

use minerva::inpututils::*;

fn main() {
    let pool = minerva::db::cria_pool_conexoes();
    let conn = pool.get().unwrap();
    let mut novocliente = NovoCliente::new();
    let mut enderecos = Vec::new();
    
    prompt("Nome: ");
    novocliente.nome = get_input();
    prompt("Tipo: ");
    novocliente.tipo = get_input().parse::<i16>().unwrap();
    prompt("PJ? (S/N) ");
    novocliente.pj = get_bool();
    prompt("Docto: ");
    novocliente.docto = get_input();

    loop {
        prompt("Adicionar mais endereços? (S/N) ");
        if !get_bool() { break; }
        
        let mut novoenderec = NovoEndereco::new();
        
        prompt("Tipo de endereço: ");
        novoenderec.tipo = get_input().parse::<i16>().unwrap();
        prompt("Logradouro: ");
        novoenderec.logradouro = get_input();
        prompt("Número: ");
        novoenderec.numero = get_input();
        prompt("Complemento (opcional): ");
        novoenderec.complemento = get_input_opt();
        prompt("Bairro: ");
        novoenderec.bairro = get_input();
        prompt("UF: ");
        novoenderec.uf = get_input();
        prompt("Cidade: ");
        novoenderec.cidade = get_input();
        enderecos.push(novoenderec);
        
        println!();
    }

    let c: Cliente =
        diesel::insert_into(cliente::table)
        .values(&novocliente)
        .get_result(&conn)
        .expect("Erro ao salvar cliente");

    for mut e in enderecos {
        e.cliente_id = c.id;
        let _: Endereco =
            diesel::insert_into(endereco::table)
            .values(&e)
            .get_result(&conn)
            .expect("Erro ao salvar endereço");
    }
}
