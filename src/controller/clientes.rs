// controller/clientes.rs -- Uma parte de Minerva.rs
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

use diesel::prelude::*;
use crate::model::schema::{ cliente, endereco };
use crate::model::cliente::{ Cliente, ClienteRepr, ClienteRecv };
use crate::model::endereco::{ Endereco, EnderecoRecv };
use super::log::*;

pub fn lista_clientes(conexao: &PgConnection, limite: i64) -> Vec<ClienteRepr> {
    let cli_req = cliente::table.limit(limite)
        .load::<Cliente>(conexao)
        .expect("Erro ao carregar clientes");
    let mut clientes = Vec::new();
    for c in cli_req {
        let enderecos = carrega_enderecos_cliente(conexao, c.id);
        let repr = ClienteRepr::from(&c, enderecos);
        clientes.push(repr);
    }
    clientes
}

pub fn get_cliente(conexao: &PgConnection, userid: i32) -> Option<ClienteRepr> {
    use crate::model::schema::cliente::dsl::*;
    let cli_req = cliente.filter(id.eq(&userid))
        .load::<Cliente>(conexao)
        .expect("Erro ao carregar cliente");
    match cli_req.first() {
        None => None,
        Some(cl) => {
            let enderecos = carrega_enderecos_cliente(conexao, cl.id);
            Some(ClienteRepr::from(&cl, enderecos))
        }
    }
}

pub fn registra_cliente(conexao: &PgConnection, dados: ClienteRecv) -> i32 {
    let (cl_recv, end_recv) = dados.into_parts();
    let c: Cliente =
        diesel::insert_into(cliente::table)
        .values(&cl_recv)
        .get_result(conexao)
        .expect("Erro ao inserir novo cliente");
    let _ = registra_log(
        conexao,
        String::from("CLIENTE"),
        String::from("TO-DO"),
        DBOperacao::Insercao,
        Some(format!("Inserindo cliente {}", c.id)));
    registra_enderecos_cliente(conexao, c.id, end_recv);
    c.id
}

pub fn deleta_cliente(conexao: &PgConnection, cl: ClienteRepr) {
    use crate::model::schema::cliente::dsl::*;
    deleta_enderecos(conexao, cl.enderecos);
    diesel::delete(cliente.filter(id.eq(&cl.id)))
        .execute(conexao)
        .expect("Erro ao deletar cliente");
}

pub fn deleta_todos(conexao: &PgConnection) -> (usize, usize) {
    let num_end = diesel::delete(endereco::table)
        .execute(conexao)
        .expect("Erro ao deletar endereços");
    let _ = registra_log(
        conexao,
        String::from("ENDERECO"),
        String::from("TO-DO"),
        DBOperacao::Remocao,
        Some(String::from("Removendo todos os endereços")));
    let num_cl = diesel::delete(cliente::table)
        .execute(conexao)
        .expect("Erro ao deletar clientes");
    let _ = registra_log(
        conexao,
        String::from("CLIENTE"),
        String::from("TO-DO"),
        DBOperacao::Remocao,
        Some(String::from("Removendo todos os clientes")));
    (num_end, num_cl)
}

fn registra_enderecos_cliente(
    conexao: &PgConnection,
    cliente_id: i32,
    enderecos: Vec<EnderecoRecv>
) {
    for e_recv in enderecos {
        let mut e = e_recv.into_new();
        e.cliente_id = cliente_id;
        let e_ins: Endereco =
            diesel::insert_into(endereco::table)
            .values(&e)
            .get_result(conexao)
            .expect("Erro ao salvar endereco");
        let _ = registra_log(
            conexao,
            String::from("ENDERECO"),
            String::from("TO-DO"),
            DBOperacao::Insercao,
            Some(format!("Inserindo endereco {}", e_ins.id)));
    }
}

fn carrega_enderecos_cliente(conexao: &PgConnection, userid: i32) -> Vec<Endereco> {
    use crate::model::schema::endereco::dsl::*;
    endereco.filter(cliente_id.eq(&userid))
        .load::<Endereco>(conexao)
        .expect("Erro ao carregar endereços")
}

fn deleta_enderecos(conexao: &PgConnection, enderecos: Vec<Endereco>) {
    use crate::model::schema::endereco::dsl::*;
    for end in enderecos {
        diesel::delete(endereco.filter(id.eq(&end.id)))
            .execute(conexao)
            .expect("Erro ao deletar endereço");
        let _ = registra_log(
            conexao,
            String::from("ENDERECO"),
            String::from("TO-DO"),
            DBOperacao::Remocao,
            Some(format!("Removendo endereco {}", end.id)));
    }
}
