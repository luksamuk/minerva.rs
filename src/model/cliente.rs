// model/cliente.rs -- Uma parte de Minerva.rs
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

use super::endereco::{Endereco, EnderecoRecv};
use crate::model::schema::cliente;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Cliente {
    pub id: i32,
    pub tipo: i16,
    pub nome: String,
    pub pj: bool,
    pub docto: String,
    pub ativo: bool,
    pub bloqueado: bool,
}

#[derive(Insertable)]
#[table_name = "cliente"]
pub struct NovoCliente {
    pub tipo: i16,
    pub nome: String,
    pub pj: bool,
    pub docto: String,
    pub ativo: bool,
    pub bloqueado: bool,
}

#[derive(Serialize)]
pub struct ClienteRepr {
    pub id: i32,
    #[serde(skip_serializing)]
    pub tipo: i16,
    pub nome: String,
    pub pj: bool,
    pub docto: String,
    pub ativo: bool,
    pub bloqueado: bool,
    pub enderecos: Vec<Endereco>,
}

#[derive(Deserialize, Clone)]
pub struct ClienteRecv {
    pub nome: String,
    pub pj: bool,
    pub docto: String,
    pub enderecos: Vec<EnderecoRecv>,
}

impl NovoCliente {
    pub fn new() -> Self {
        Self {
            tipo: 0,
            nome: String::new(),
            pj: false,
            docto: String::new(),
            ativo: true,
            bloqueado: false,
        }
    }
}

impl ClienteRepr {
    pub fn from(cl: &Cliente, enderec: Vec<Endereco>) -> Self {
        Self {
            id: cl.id,
            tipo: cl.tipo,
            nome: cl.nome.clone(),
            pj: cl.pj,
            docto: cl.docto.clone(),
            ativo: true,
            bloqueado: false,
            enderecos: enderec,
        }
    }
}

impl ClienteRecv {
    pub fn into_parts(&self) -> (NovoCliente, Vec<EnderecoRecv>) {
        (
            NovoCliente {
                tipo: 0,
                nome: self.nome.clone(),
                pj: self.pj,
                docto: self.docto.clone(),
                ativo: true,
                bloqueado: false,
            },
            self.enderecos.clone(),
        )
    }
}
