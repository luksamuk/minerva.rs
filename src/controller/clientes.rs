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

//! Ferramentas para tráfego de dados entre as rotas de clientes e o banco de
//! dados.
//! 
//! As ferramentas deste módulo realizam o tráfego entre os dados recebidos
//! através das rotas de clientes e a tabela `cliente` e relacionadas. Para as
//! regras de negócio da aplicação, veja [`bo::clientes`][`crate::bo::clientes`].

use super::log::*;
use crate::model::cliente::*;
use crate::model::endereco::*;
use crate::model::schema::{cliente, endereco};
use diesel::prelude::*;

/// Lista uma quantidade limitada de clientes cadastrados no sistema.
/// 
/// Retorna um Vec com estruturas que representam os dados de um cliente,
/// incluindo os endereços cadastrados para o mesmo. A quantidade de clientes
/// retornados não será superior à informada no argumento `limite`.
pub fn lista_clientes(conexao: &PgConnection, limite: i64) -> Vec<ClienteRepr> {
    let cli_req = cliente::table
        .limit(limite)
        .load::<Cliente>(conexao)
        .expect("Erro ao carregar clientes");
    let mut clientes = Vec::new();
    for c in cli_req {
        let enderecos = carrega_enderecos_cliente(conexao, c.id);
        let repr = ClienteRepr::from((&c, enderecos));
        clientes.push(repr);
    }
    clientes
}

/// Retorna os dados de um cliente cadastrado no sistema.
/// 
/// Será retornado um `Option` que poderá conter uma estrutura que representa
/// os dados de um único cliente, incluindo os endereços cadastrados para o
/// mesmo. O cliente será procurado de acordo com o seu id repassado no
/// argumento `userid`.
pub fn get_cliente(conexao: &PgConnection, userid: i32) -> Option<ClienteRepr> {
    use crate::model::schema::cliente::dsl::*;
    let cli_req = cliente
        .filter(id.eq(&userid))
        .load::<Cliente>(conexao)
        .expect("Erro ao carregar cliente");
    match cli_req.first() {
        None => None,
        Some(cl) => {
            let enderecos = carrega_enderecos_cliente(conexao, cl.id);
            Some(ClienteRepr::from((cl, enderecos)))
        }
    }
}

/// Registra um novo cliente no banco de dados.
/// 
/// Esta função toma os dados do cliente recebidos através de uma requisição
/// POST, e cadastra-os no banco de dados. Os dados recebidos não são avaliados
/// quanto à sua validade, sendo inseridos diretamente no banco de dados.
/// Será retornado o id do cliente após ser cadastrado no banco de dados.
pub fn registra_cliente(conexao: &PgConnection, dados: ClienteRecv) -> i32 {
    let (cl_recv, end_recv) = dados.into();
    let c: Cliente = diesel::insert_into(cliente::table)
        .values(&cl_recv)
        .get_result(conexao)
        .expect("Erro ao inserir novo cliente");
    let _ = registra_log(
        conexao,
        String::from("CLIENTE"),
        String::from("TO-DO"),
        DBOperacao::Insercao,
        Some(format!("Cliente {}", c.id)),
    );
    registra_enderecos_cliente(conexao, c.id, end_recv);
    c.id
}

/// Deleta um cliente em específico no banco de dados.
/// 
/// O cliente a ser deletado deverá ser informado através de uma estrutura
/// completa de representação do mesmo, posto que sua remoção também envolverá
/// remoção de todos os endereços associados ao mesmo. A função assume que os
/// dados de cliente passados sejam válidos.
pub fn deleta_cliente(conexao: &PgConnection, cl: ClienteRepr) {
    use crate::model::schema::cliente::dsl::*;
    deleta_enderecos(conexao, cl.enderecos);
    diesel::delete(cliente.filter(id.eq(&cl.id)))
        .execute(conexao)
        .expect("Erro ao deletar cliente");
}

/// Deleta todos os clientes do banco de dados.
/// 
/// Esta função varre todos os dados de clientes e endereços do banco de dados,
/// retornando uma tuple contendo, respectivamente, as quantidades de registros
/// de usuários e de endereços deletados neste processo.
/// Utilize esta função com cuidado.
pub fn deleta_todos(conexao: &PgConnection) -> (usize, usize) {
    let num_end = diesel::delete(endereco::table)
        .execute(conexao)
        .expect("Erro ao deletar endereços");
    let _ = registra_log(
        conexao,
        String::from("ENDERECO"),
        String::from("TO-DO"),
        DBOperacao::Remocao,
        Some(String::from("Removendo todos os endereços")),
    );
    let num_cl = diesel::delete(cliente::table)
        .execute(conexao)
        .expect("Erro ao deletar clientes");
    let _ = registra_log(
        conexao,
        String::from("CLIENTE"),
        String::from("TO-DO"),
        DBOperacao::Remocao,
        Some(String::from("Removendo todos os clientes")),
    );
    (num_end, num_cl)
}

/// Registra os dados de endereços para um cliente em específico.
/// 
/// Esta função assume que os dados de endereços recebidos estejam corretos,
/// e também assume que o cliente, cujo id tenha sido informado via parâmetro,
/// já tenha sido inserido no banco de dados.
fn registra_enderecos_cliente(
    conexao: &PgConnection,
    cliente_id: i32,
    enderecos: Vec<EnderecoRecv>,
) {
    for e_recv in enderecos {
        let mut e: NovoEndereco = e_recv.into();
        e.cliente_id = cliente_id;
        let e_ins: Endereco = diesel::insert_into(endereco::table)
            .values(&e)
            .get_result(conexao)
            .expect("Erro ao salvar endereco");
        let _ = registra_log(
            conexao,
            String::from("ENDERECO"),
            String::from("TO-DO"),
            DBOperacao::Insercao,
            Some(format!("Endereço {}", e_ins.id)),
        );
    }
}

/// Recupera uma coleção de endereços para um cliente em específico.
/// 
/// Esta função procurará pelos endereços que apontem para o cliente cujo id
/// foi informado via parâmetro.
fn carrega_enderecos_cliente(conexao: &PgConnection, userid: i32) -> Vec<Endereco> {
    use crate::model::schema::endereco::dsl::*;
    endereco
        .filter(cliente_id.eq(&userid))
        .load::<Endereco>(conexao)
        .expect("Erro ao carregar endereços")
}

/// Deleta todos os endereços referenciados.
/// 
/// Esta função requisita os dados completos de endereço de um cliente, que
/// deverão ser repassados integralmente.
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
            Some(format!("Endereço {}", end.id)),
        );
    }
}
