// model/estoque.rs -- Uma parte de Minerva.rs
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

//! Utilitários de modelagem de estoque e movimentações de estoque para banco de
//! dados e regras de negócio.
//! 
//! Este módulo define estruturas para o tráfego de dados de posição e
//! movimentação de estoque entre as partes respectivas do sistema.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

use super::schema::{estoque, mov_estoque};

/// Representa a posição de estoque de um produto, como armazenada no banco de
/// dados, na tabela `estoque`.
#[derive(Queryable, Insertable, Clone, Identifiable, Serialize, Deserialize)]
#[table_name = "estoque"]
#[primary_key(produto_id)]
pub struct Estoque {
    /// Id do produto associado à posição de estoque, na tabela `produto`.
    pub produto_id: i32,
    /// Quantidade do produto em estoque. Admite até três casas decimais.
    /// Valor máximo: `999999999.999`.
    pub quantidade: BigDecimal,
    /// Preço unitário atual do produto. Não poderá ser menor ou igual a zero.
    /// Admite até quatro casas decimais. Valor máximo: `999999999.9999`.
    pub precounitario: BigDecimal,
}

/// Representa uma movimentação no estoque de um produto, da forma como é
/// armazenada na tabela `mov_estoque`.
#[derive(Queryable, Clone, Identifiable, Serialize)]
#[table_name = "mov_estoque"]
pub struct MovEstoque {
    /// Id da movimentação de estoque no banco de dados.
    pub id: i32,
    /// Id do produto cujo estoque foi movimentado, na tabela `produto`.
    pub produto_id: i32,
    /// Documento relacionado à movimentação de estoque (ex: número da nota
    /// fiscal).
    pub docto: String,
    /// Quantidade de produto movimentada. Positivo para entrada de estoque,
    /// negativo para saída de estoque. Admite até três casas decimais.
    /// Valor máximo: `999999999.999`.
    pub quantidade: BigDecimal,
    /// Preço do frete da quantidade de produto movimentada no estoque, quando
    /// aplicável. Movimentações sem preço de frete serão armazenadas com valor
    /// zero. Admite até quatro casas decimais. Valor máximo: `999999999.9999`.
    pub preco_frete: BigDecimal,
    /// Data e hora de registro da movimentação de estoque no sistema.
    pub datahora: DateTime<chrono::Utc>,
    /// Novo preço unitário atribuído ao produto pela movimentação. Não poderá
    /// ser menor ou igual a zero. Admite até quatro casas decimais.
    /// Valor máximo: `999999999.9999`.
    pub preco_unitario: BigDecimal,
}

/// Representa os dados de uma movimentação de estoque a ser inserida no banco
/// de dados.
#[derive(Insertable, Clone)]
#[table_name = "mov_estoque"]
pub struct NovoMovEstoque {
    /// Id do produto cujo estoque será movimentado, na tabela `produto`.
    /// Ver [`MovEstoque::produto_id`].
    pub produto_id: i32,
    /// Documento relacionado à movimentação de estoque.
    /// Ver [`MovEstoque::docto`].
    pub docto: String,
    /// Quantidade de produto movimentada. Pode ser positivo ou negativo.
    /// Ver [`MovEstoque::quantidade`].
    pub quantidade: BigDecimal,
    /// Novo preço unitário do produto.
    /// Ver [`MovEstoque::preco_unitario`].
    pub preco_unitario: BigDecimal,
    /// Preço do frete da quantidade de produto a ser movimentada, se aplicável.
    /// Ver [`MovEstoque::preco_frete`].
    pub preco_frete: BigDecimal,
    /// Data e hora de registro da movimentação de estoque.
    /// Ver [`MovEstoque::datahora`].
    pub datahora: DateTime<chrono::Utc>,
}

/// Representa os dados de uma movimentação de estoque a serem recebidos como
/// corpo de uma requisição.
/// 
/// Uma movimentação de estoque a ser realizada deverá ser recebida com uma
/// estrutura similar à seguir, em JSON:
/// 
/// ```json
/// {
///   "produto_id": 8,
///   "docto": "00000000",
///   "quantidade": 200.0,
///   "preco_frete": 15.00,
///   "preco_unitario": 1.70
/// }
/// ```
/// 
/// Note que, por mais que esteja presente no exemplo, o preço do frete poderá
/// ser omitido ou declarado com valor `null`.
/// 
/// A data e a hora da movimentação de estoque são registradas no momento em que
/// estes dados são tratados para a inserção no banco de dados, caso não haja
/// regras de negócio que impeçam este processo. Por isso, a data e hora
/// registradas serão aproximadamente as mesmas do momento em que a requisição
/// de movimentação de estoque é recebida via web.
/// 
/// Para mais informações, veja [`MovEstoque`].
#[derive(Deserialize, Clone)]
pub struct MovEstoqueRecv {
    /// Id do produto cujo estoque será movimentado, na tabela `produto`.
    /// Ver [`MovEstoque::produto_id`].
    pub produto_id: i32,
    /// Documento relacionado à movimentação de estoque.
    /// Ver [`MovEstoque::docto`].
    pub docto: String,
    /// Quantidade de produto movimentada. Pode ser positivo ou negativo.
    /// Ver [`MovEstoque::quantidade`].
    pub quantidade: BigDecimal,
    /// Novo preço unitário do produto.
    /// Ver [`MovEstoque::preco_unitario`].
    pub preco_unitario: BigDecimal,
    /// Preço do frete da quantidade de produto a ser movimentada, se aplicável.
    /// Opcional. Ver [`MovEstoque::preco_frete`].
    pub preco_frete: Option<BigDecimal>,
}

/// Representa uma união entre os dados de um produto e os dados de estoque do
/// produto referido.
/// 
/// Para mais informações, ver [`Estoque`] e [`Produto`][`super::produto::Produto`].
#[derive(Serialize, Clone)]
pub struct EstoqueRepr {
    /// Id do produto na tabela `produto` no banco de dados.
    /// Ver [`Produto::id`][`super::produto::Produto::id`].
    pub id: i32,
    /// Descrição do produto.
    /// Ver [`Produto::descricao`][`super::produto::Produto::descricao`].
    pub descricao: String,
    /// Unidade de saída do produto.
    /// Ver [`Produto::unidsaida`][`super::produto::Produto::unidsaida`].
    pub unidsaida: String,
    /// Quantidade em estoque do produto.
    /// Ver [`Estoque::quantidade`].
    pub quantidade: BigDecimal,
    /// Preço unitário do produto.
    /// Ver [`Estoque::precounitario`].
    pub preco_unitario: BigDecimal,
}

impl From<MovEstoqueRecv> for NovoMovEstoque {
    /// Realiza uma conversão de dados de uma movimentação de estoque, recebidos
    /// via requisição web, para dados prontos para serem inseridos no banco de
    /// dados.
    /// 
    /// Caso nenhum preço de frete tenha sido informado, o valor será definido
    /// como zero. A data e hora do movimento serão atribuídas no momento desse
    /// processo de conversão.
    fn from(recv: MovEstoqueRecv) -> Self {
        Self {
            produto_id: recv.produto_id,
            docto: recv.docto.clone(),
            quantidade: recv.quantidade.clone(),
            preco_unitario: recv.preco_unitario.clone(),
            preco_frete: match recv.preco_frete {
                Some(frete) => frete,
                None => BigDecimal::from_str("0.0000").unwrap(),
            },
            datahora: chrono::offset::Utc::now(),
        }
    }
}
