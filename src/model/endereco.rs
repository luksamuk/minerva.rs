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

//! Utilitários de modelagem de endereço para banco de dados e regras de
//! negócio.
//!
//! Este módulo define estruturas para o tráfego de dados de endereço entre
//! as partes respectivas do sistema.

use super::schema::endereco;
use serde::{Deserialize, Serialize};

/// Representa a estrutura de um elemento da tabela `endereco` do banco de
/// dados e sua representação como resposta a uma requisição.
///
/// Esta estrutura também pode ser utilizada para representação dos dados de um
/// endereço no retorno a uma requisição.
#[derive(Queryable, Serialize)]
pub struct Endereco {
    /// Id do endereço no banco de dados.
    /// Este campo não será mostrado na resposta a uma requisição.
    #[serde(skip_serializing)]
    pub id: i32,
    /// Id do cliente no banco de dados ao qual o endereço encontra-se
    /// associado.
    /// Este campo não será mostrado na resposta a uma requisição.
    #[serde(skip_serializing)]
    pub cliente_id: i32,
    /// Tipo do endereço. Definido como 0 por padrão.
    /// Este campo não será mostrado na resposta a uma requisição.
    #[serde(skip_serializing)]
    pub tipo: i16,
    /// Logradouro.
    pub logradouro: String,
    /// Número do endereço. Pode ser representado como apenas dígitos e também
    /// pode admitir números de apartamento. Ex: 123-A.
    pub numero: String,
    /// Complemento do endereço. Ex: Casa, Apartamento.
    /// Não precisa ser informado.
    pub complemento: Option<String>,
    /// Bairro.
    pub bairro: String,
    /// Unidade Federativa. Preferencialmente deve ser armazenada com duas
    /// letras. Ex: MG, SP.
    pub uf: String,
    /// Cidade.
    pub cidade: String,
}

/// Representa os dados de um endereço a serem inseridos na criação de um novo
/// endereço no banco de dados.
#[derive(Insertable, Deserialize, Clone, Default)]
#[table_name = "endereco"]
pub struct NovoEndereco {
    /// Id do cliente associado. Ver [`Endereco::cliente_id`].
    pub cliente_id: i32,
    /// Tipo do endereco. Ver [`Endereco::tipo`].
    pub tipo: i16,
    /// Logradouro. Ver [`Endereco::logradouro`].
    pub logradouro: String,
    /// Número do endereço. Ver [`Endereco::numero`].
    pub numero: String,
    /// Complemento do endereço. Não precisa ser informado.
    /// Ver [`Endereco::complemento`].
    pub complemento: Option<String>,
    /// Bairro. Ver [`Endereco::bairro`].
    pub bairro: String,
    /// Unidade Federativa. Ver [`Endereco::uf`].
    pub uf: String,
    /// Cidade. Ver [`Endereco::cidade`].
    pub cidade: String,
}

/// Representa os dados de um endereço a ser cadastrado, recebidos como corpo de
/// uma requisição.
///
/// Note que os dados do endereço não envolvem o id do cliente associado no
/// banco, mas espera-se que este dado esteja disponível após o cadastro do
/// mesmo.
///
/// Esta estrutura deverá estar associada a uma estrutura
/// [`ClienteRecv`][`super::cliente::ClienteRecv`], que será deserializada a
/// partir dos dados recebidos pela requisição web. Estes dados envolvem um
/// vetor de estruturas `EnderecoRecv`.
#[derive(Deserialize, Clone)]
pub struct EnderecoRecv {
    /// Logradouro. Ver [`Endereco::logradouro`].
    pub logradouro: String,
    /// Número do endereço. Ver [`Endereco::numero`].
    pub numero: String,
    /// Complemento do endereço. Não precisa ser informado.
    /// Ver [`Endereco::complemento`].
    pub complemento: Option<String>,
    /// Bairro. Ver [`Endereco::bairro`].
    pub bairro: String,
    /// Unidade Federativa. Ver [`Endereco::uf`].
    pub uf: String,
    /// Cidade. Ver [`Endereco::cidade`].
    pub cidade: String,
}

impl NovoEndereco {
    /// Cria uma estrutura de cadastro de endereço sem dados significativos.
    /// A estrutura não será criada automaticamente no banco de dados.
    ///
    /// A estrutura criada para representar um novo endereço vem, por padrão,
    /// com os seguintes dados:
    /// - Id do cliente associado: -1 (inválido);
    /// - Tipo do endereço: 0 (padrão);
    /// - Logradouro: vazio;
    /// - Número: vazio;
    /// - Complemento: `None`;
    /// - Bairro: vazio;
    /// - UF: vazio;
    /// - cidade: vazio.
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

impl From<EnderecoRecv> for NovoEndereco {
    /// Gera uma estrutura de cadastro de um novo endereço a partir de uma
    /// estrutura de dados de endereço, recebidos via requisição. A estrutura
    /// não possuirá id válido para um cliente associado.
    ///
    /// Esta função é particularmente útil para converter os dados de um
    /// endereço, quando recebidos via requisição web, em dados que possam ser
    /// utilizados em cadastro no banco de dados.
    ///
    /// A estrutura resultante não possuirá um campo `cliente_id` válido, e
    /// portanto o mesmo deverá ser preenchido após o cadastro do cliente
    /// referido, de onde recuperar-se-á o id associado.
    fn from(recv: EnderecoRecv) -> Self {
        NovoEndereco {
            cliente_id: -1,
            tipo: 0,
            logradouro: recv.logradouro.clone(),
            numero: recv.numero.clone(),
            complemento: recv.complemento.clone(),
            bairro: recv.bairro.clone(),
            uf: recv.uf.clone(),
            cidade: recv.cidade.clone(),
        }
    }
}
