// model/endereco.rs -- Uma parte de Minerva.rs
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

use super::schema::endereco;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize)]
pub struct Endereco {
    #[serde(skip_serializing)]
    pub id: i32,
    #[serde(skip_serializing)]
    pub cliente_id: i32,
    #[serde(skip_serializing)]
    pub tipo: i16,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub uf: String,
    pub cidade: String,
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name="endereco"]
pub struct NovoEndereco {
    pub cliente_id: i32,
    pub tipo: i16,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub uf: String,
    pub cidade: String,
}

#[derive(Deserialize, Clone)]
pub struct EnderecoRecv {
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub uf: String,
    pub cidade: String,
}

impl NovoEndereco {
    pub fn new() -> Self {
        Self {
            cliente_id: -1,
            tipo: 0,
            logradouro: String::new(),
            numero: String::new(),
            complemento: None,
            bairro: String::new(),
            uf: String::new(),
            cidade: String::new(),
        }
    }
}

impl EnderecoRecv {
    pub fn into_new(&self) -> NovoEndereco {
        NovoEndereco {
            cliente_id: -1,
            tipo: 0,
            logradouro: self.logradouro.clone(),
            numero: self.numero.clone(),
            complemento: self.complemento.clone(),
            bairro: self.bairro.clone(),
            uf: self.uf.clone(),
            cidade: self.cidade.clone(),
        }
    }
}
