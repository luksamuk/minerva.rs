// model/produto.rs -- Uma parte de Minerva.rs
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

//! Utilitários de modelagem de produto para banco de dados e regras de negócio.
//!
//! Este módulo define estruturas para o tráfego de dados de produtos entre as
//! partes respectivas do sistema.
//!
//! O model de produtos não compreende dados relacionados a controle de estoque.
//! Para tanto, veja o módulo [`estoque`][`super::estoque`].

use super::schema::produto;
use serde::{Deserialize, Serialize};

/// Representa os dados de um produto armazenados no banco de dados.
///
/// Os dados de um produto compreendem, em sua maioria, informações básicas a
/// serem armazenadas uma única vez, na tabela `produto`.
#[derive(Identifiable, Queryable, Serialize, Debug, Clone)]
#[table_name = "produto"]
pub struct Produto {
    /// Id do produto no banco de dados.
    pub id: i32,
    /// Descrição textual do produto.
    pub descricao: String,
    /// Unidade de saída do produto. Deve ser armazenada em uppercase.
    /// Ex: UN/UNID (Unidade), KG (Quilograma), FD (Fardo), L (Litro), etc.
    pub unidsaida: String,
}

/// Representa os dados de inserção de um novo produto no banco de dados.
///
/// A inserção do produto envolve também o recebimento desses dados via
/// requisição POST na respectiva rota de cadastro de produtos, e deverá ser
/// feita de acordo com o exemplo a seguir, em JSON:
///
/// ```json
/// {
///   "descricao": "Produto adicionado via requisição web",
///   "unidsaida": "KG"
/// }
/// ```
#[derive(Debug, Insertable, Deserialize, Clone, Default)]
#[table_name = "produto"]
pub struct NovoProduto {
    /// Descrição textual do produto.
    /// Ver [`Produto::descricao`].
    pub descricao: String,
    /// Unidade de saída do produto.
    /// Ver [`Produto::unidsaida`].
    pub unidsaida: String,
}

impl NovoProduto {
    /// Cria um novo produto com dados iniciais inválidos.
    ///
    /// O produto retornado terá sua descrição e unidade de saída em branco.
    pub fn new() -> Self {
        Self {
            descricao: String::new(),
            unidsaida: String::new(),
        }
    }
}
