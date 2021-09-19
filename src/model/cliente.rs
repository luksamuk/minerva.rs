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

//! Utilitários de modelagem de cliente para banco de dados e regras de negócio.
//!
//! Este módulo define estruturas para o tráfego de dados de clientes entre as
//! partes respectivas do sistema.

use super::endereco::{Endereco, EnderecoRecv};
use crate::model::schema::cliente;
use serde::{Deserialize, Serialize};

/// Representa a estrutura de um elemento da tabela `cliente` do banco de dados.
#[derive(Queryable)]
pub struct Cliente {
    /// Id do cliente no banco.
    pub id: i32,
    /// Tipo do cliente. Definido como 0 por padrão.
    pub tipo: i16,
    /// Nome do cliente.
    pub nome: String,
    /// Determina se o cliente é uma pessoa jurídica. Caso seja, assume valor
    /// verdadeiro. Caso seja pessoa física, assume valor falso.
    pub pj: bool,
    /// Documento do cliente. Caso seja pessoa física, será seu CPF com onze
    /// dígitos, pontos e hífens. Caso seja pessoa jurídica, será seu CNPJ
    /// com catorze dígitos, pontos, hífens e barras.
    pub docto: String,
    /// Determina se o cliente está ativo. Um cliente pode ser definido como
    /// inativo se sua remoção não for conveniente.
    pub ativo: bool,
    /// Determina se o cliente está bloqueado. Um cliente bloqueado não poderá
    /// ter operações feitas em seu nome.
    pub bloqueado: bool,
}

/// Representa os dados de um cliente a serem inseridos na criação de um novo
/// cliente no banco de dados.
#[derive(Insertable, Default)]
#[table_name = "cliente"]
pub struct NovoCliente {
    /// Tipo do cliente. Ver [`Cliente::id`].
    pub tipo: i16,
    /// Nome do cliente. Ver [`Cliente::nome`].
    pub nome: String,
    /// Determina se o cliente é uma pessoa jurídica. Ver [`Cliente::pj`].
    pub pj: bool,
    /// Documento do cliente. Ver [`Cliente::docto`].
    pub docto: String,
    /// Determina se o cliente está ativo. Ver [`Cliente::ativo`].
    pub ativo: bool,
    /// Determina se o cliente está bloqueado. Ver [`Cliente::bloqueado`].
    pub bloqueado: bool,
}

/// Representa os dados de um cliente a serem retornados como resposta a uma
/// requisição.
///
/// Uma resposta a uma requisição de um cliente será mostrada com uma estrutura
/// similar à estrutura a seguir, em JSON:
///
/// ```json
/// {
///   "id": 7,
///   "nome": "Zé da Silva",
///   "pj": false,
///   "docto": "999.999.999-99",
///   "ativo": true,
///   "bloqueado": false,
///   "enderecos": [
///     {
///       "logradouro": "Rua dos Tolos",
///       "numero": "0",
///       "complemento": "Casa",
///       "bairro": "Nenhum",
///       "uf": "MG",
///       "cidade": "Cidade"
///     }
///   ]
/// }
/// ```
///
/// Note que a lista de endereços a serem retornados, que pode ser vazia,
/// relaciona-se diretamente à estrutura que representa os dados de um endereço
/// registrado no banco de dados. Portanto, veja também
/// [`Endereco`][`super::endereco::Endereco`].
#[derive(Serialize)]
pub struct ClienteRepr {
    /// Id do cliente no banco de dados.
    pub id: i32,
    /// Tipo do cliente. Ver [`Cliente::id`].
    /// Este campo não será mostrado na resposta a uma requisição, por enquanto.
    #[serde(skip_serializing)]
    pub tipo: i16,
    /// Nome do cliente. Ver [`Cliente::nome`].
    pub nome: String,
    /// Determina se o cliente é uma pessoa jurídica. Ver [`Cliente::pj`].
    pub pj: bool,
    /// Documento do cliente. Ver [`Cliente::docto`].
    pub docto: String,
    /// Determina se o cliente está ativo. Ver [`Cliente::ativo`].
    pub ativo: bool,
    /// Determina se o cliente está bloqueado. Ver [`Cliente::bloqueado`].
    pub bloqueado: bool,
    /// Lista de endereços registrados para o cliente.
    /// Ver [`Endereco`][`super::endereco::Endereco`].
    pub enderecos: Vec<Endereco>,
}

/// Representa os dados de um cliente a ser cadastrado, recebidos como corpo de
/// uma requisição.
///
/// Um cliente a ser cadastrado deve ser recebido via requisição utilizando uma
/// estrutura similar à seguir, em JSON:
///
/// ```json
/// {
///   "nome": "Zé da Silva",
///   "pj": false,
///   "docto": "999.999.999-99",
///   "enderecos": [
///     {
///       "logradouro": "Rua dos Tolos",
///       "numero": "0",
///       "complemento": "Casa",
///       "bairro": "Nenhum",
///       "uf": "MG",
///       "cidade": "Cidade"
///     }
///   ]
/// }
/// ```
///
/// Note que a lista de endereços a serem informados, que pode ser vazia,
/// relaciona-se diretamente à estrutura que representa os dados de um endereço
/// a serem recebidos via requisição. Portanto, veja também
/// [`EnderecoRecv`][`super::endereco::EnderecoRecv`].
#[derive(Deserialize, Clone)]
pub struct ClienteRecv {
    /// Nome do cliente. Ver [`Cliente::nome`].
    pub nome: String,
    /// Determina se o cliente é uma pessoa jurídica. Ver [`Cliente::pj`].
    pub pj: bool,
    /// Documento do cliente. Ver [`Cliente::docto`].
    pub docto: String,
    /// Lista de endereços a serem registrados para o cliente.
    /// Ver [`EnderecoRecv`][`super::endereco::EnderecoRecv`].
    pub enderecos: Vec<EnderecoRecv>,
}

impl NovoCliente {
    /// Cria uma estrutura de cadastro de cliente sem dados significativos.
    /// A estrutura não será criada automaticamente no banco de dados.
    ///
    /// A estrutura criada para representar um novo cliente vem, por padrão,
    /// com os seguintes dados:
    /// - Tipo do cliente: 0 (padrão);
    /// - Nome do cliente: vazio;
    /// - PJ: `false`;
    /// - Documento: vazio;
    /// - Ativo: `true`;
    /// - Bloqueado: `false`.
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
    /// Gera uma representação de um cliente a partir de uma referência a uma
    /// estrutura de [`Cliente`] e um vetor de estruturas de
    /// [`Endereco`][`super::endereco::Endereco`].
    ///
    /// Esta função é particularmente útil quando um cliente e seus endereços
    /// associados forem requisitados a partir de uma query, e for necessário
    /// reunir estas informações sob uma mesma estrutura (por exemplo, para
    /// serialização e subsequente retorno em formato JSON a uma requisição.
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
    /// Divide uma representação de dados recebidos para um cadastro de um
    /// cliente em uma tuple contendo suas respectivas partes separadas.
    ///
    /// Esta função é particularmente útil para separar os dados recebidos
    /// via requisição em estruturas prontas ou quase prontas para realização
    /// de cadastro e validações.
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
