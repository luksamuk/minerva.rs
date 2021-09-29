// bo/clientes.rs -- Uma parte de Minerva.rs
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

use regex::Regex;
use crate::model::cliente::ClienteRecv;

const ESTADOS: [&str; 27] = [
    "AC", "AL", "AM", "AP", "BA", "CE", "DF", "ES", "GO", "MA", "MG", "MS",
    "MT", "PA", "PB", "PE", "PI", "PR", "RJ", "RN", "RO", "RR", "RS", "SC",
    "SE", "SP", "TO"
];

/// Expressão regular representando um CPF no formato 999.999.999-99.
/// Os dígitos foram discriminados separadamente para facilitar na captura.
const CPF_REGEX: &str = r"^(\d)(\d)(\d).(\d)(\d)(\d).(\d)(\d)(\d)-(\d)(\d)$";

/// Expressão regular representando um CNPJ no formato 99.999.999/9999-99.
/// Os dígitos foram discriminados separadamente para facilitar na captura.
const CNPJ_REGEX: &str =
    r"^(\d)(\d).(\d)(\d)(\d).(\d)(\d)(\d)/(\d)(\d)(\d)(\d)-(\d)(\d)$";

/// Informa se um CNPJ é válido. O CNPJ deve ser repassado como um string slice,
/// sem espaços extras, e com formato adequado. Veja [CNPJ_REGEX].
fn valida_cnpj(cnpj: &str) -> bool {
    let re = Regex::new(CNPJ_REGEX).unwrap();
    let captures = match re.captures(cnpj) {
        Some(c) => c,
        None => return false,
    };

    let calcula_digito_cnpj = |digitos: Vec<i32>| {
        let parcial = digitos.iter().sum::<i32>() % 11;
        if parcial < 2 {
            0
        } else {
            11 - parcial
        }
    };

    let primeiro_digito = calcula_digito_cnpj(vec![
        captures[1].parse::<i32>().unwrap()  * 5,
        captures[2].parse::<i32>().unwrap()  * 4,
        captures[3].parse::<i32>().unwrap()  * 3,
        captures[4].parse::<i32>().unwrap()  * 2,
        captures[5].parse::<i32>().unwrap()  * 9,
        captures[6].parse::<i32>().unwrap()  * 8,
        captures[7].parse::<i32>().unwrap()  * 7,
        captures[8].parse::<i32>().unwrap()  * 6,
        captures[9].parse::<i32>().unwrap()  * 5,
        captures[10].parse::<i32>().unwrap() * 4,
        captures[11].parse::<i32>().unwrap() * 3,
        captures[12].parse::<i32>().unwrap() * 2,
    ]);

    if primeiro_digito != captures[13].parse::<i32>().unwrap() {
        return false;
    }

    let segundo_digito = calcula_digito_cnpj(vec![
        captures[1].parse::<i32>().unwrap()  * 6,
        captures[2].parse::<i32>().unwrap()  * 5,
        captures[3].parse::<i32>().unwrap()  * 4,
        captures[4].parse::<i32>().unwrap()  * 3,
        captures[5].parse::<i32>().unwrap()  * 2,
        captures[6].parse::<i32>().unwrap()  * 9,
        captures[7].parse::<i32>().unwrap()  * 8,
        captures[8].parse::<i32>().unwrap()  * 7,
        captures[9].parse::<i32>().unwrap()  * 6,
        captures[10].parse::<i32>().unwrap() * 5,
        captures[11].parse::<i32>().unwrap() * 4,
        captures[12].parse::<i32>().unwrap() * 3,
        primeiro_digito * 2,
    ]);

    segundo_digito == captures[14].parse::<i32>().unwrap()
}

#[test]
fn validacao_de_cnpj() {
    // CNPJs gerados em https://www.4devs.com.br/gerador_de_cnpj

    // CNPJs corretos
    assert!(valida_cnpj("17.578.468/0001-60"));
    assert!(valida_cnpj("26.440.024/0001-55"));
    assert!(valida_cnpj("31.060.467/0001-50"));
    assert!(valida_cnpj("88.216.800/0001-95"));

    // CNPJs com um dígito verificador incorreto
    assert!(!valida_cnpj("17.578.468/0001-65"));
    assert!(!valida_cnpj("26.440.024/0001-50"));
    assert!(!valida_cnpj("31.060.467/0001-55"));
    assert!(!valida_cnpj("88.216.800/0001-90"));

    // CNPJs com dois dígitos verificadores incorretos
    assert!(!valida_cnpj("17.578.468/0001-95"));
    assert!(!valida_cnpj("26.440.024/0001-50"));
    assert!(!valida_cnpj("31.060.467/0001-55"));
    assert!(!valida_cnpj("88.216.800/0001-60"));

    // Textos aleatórios
    assert!(!valida_cnpj(""));
    assert!(!valida_cnpj("aleatório"));
    assert!(!valida_cnpj("texto"));
    assert!(!valida_cnpj("teste"));
    assert!(!valida_cnpj("88216800000160")); // CNPJ sem pontos e barras

    // CPFs aleatórios
    assert!(!valida_cnpj("641.453.510-96"));
    assert!(!valida_cnpj("499.225.140-44"));
    assert!(!valida_cnpj("390.083.090-84"));
    assert!(!valida_cnpj("314.119.650-86"));
}

/// Informa se um CPF é válido. O CPF deve ser repassado como um string slice,
/// sem espaços extras, e com formato adequado. Veja [CPF_REGEX].
fn valida_cpf(cpf: &str) -> bool {
    let re = Regex::new(CPF_REGEX).unwrap();
    let captures = match re.captures(cpf) {
        Some(c) => c,
        None => return false,
    };

    let calcula_digito_cpf = |digitos: Vec<i32>| {
        let digito = 11 - (digitos.iter().sum::<i32>() % 11);
        if digito > 9 {
            0
        } else {
            digito
        }     
    };

    let primeiro_digito = calcula_digito_cpf(vec![
        captures[1].parse::<i32>().unwrap() * 10,
        captures[2].parse::<i32>().unwrap() * 9,
        captures[3].parse::<i32>().unwrap() * 8,
        captures[4].parse::<i32>().unwrap() * 7,
        captures[5].parse::<i32>().unwrap() * 6,
        captures[6].parse::<i32>().unwrap() * 5,
        captures[7].parse::<i32>().unwrap() * 4,
        captures[8].parse::<i32>().unwrap() * 3,
        captures[9].parse::<i32>().unwrap() * 2,
    ]);

    if primeiro_digito != captures[10].parse::<i32>().unwrap() {
        return false;
    }

    let segundo_digito = calcula_digito_cpf(vec![
        captures[1].parse::<i32>().unwrap() * 11,
        captures[2].parse::<i32>().unwrap() * 10,
        captures[3].parse::<i32>().unwrap() * 9,
        captures[4].parse::<i32>().unwrap() * 8,
        captures[5].parse::<i32>().unwrap() * 7,
        captures[6].parse::<i32>().unwrap() * 6,
        captures[7].parse::<i32>().unwrap() * 5,
        captures[8].parse::<i32>().unwrap() * 4,
        captures[9].parse::<i32>().unwrap() * 3,
        primeiro_digito * 2,
    ]);

    segundo_digito == captures[11].parse::<i32>().unwrap()
}

#[test]
fn validacao_de_cpf() {
    // CPFs gerados em https://www.4devs.com.br/gerador_de_cpf

    // CPFs corretos
    assert!(valida_cpf("641.453.510-96"));
    assert!(valida_cpf("499.225.140-44"));
    assert!(valida_cpf("390.083.090-84"));
    assert!(valida_cpf("314.119.650-86"));

    // CPFs com um dígito verificador incorreto
    assert!(!valida_cpf("641.453.510-94"));
    assert!(!valida_cpf("499.225.140-46"));
    assert!(!valida_cpf("390.083.090-86"));
    assert!(!valida_cpf("314.119.650-84"));

    // CPFs com dois dígitos verificadores incorretos
    assert!(!valida_cpf("641.453.510-84"));
    assert!(!valida_cpf("499.225.140-86"));
    assert!(!valida_cpf("390.083.090-46"));
    assert!(!valida_cpf("314.119.650-94"));

    // Textos aleatórios
    assert!(!valida_cpf(""));
    assert!(!valida_cpf("aleatório"));
    assert!(!valida_cpf("texto"));
    assert!(!valida_cpf("teste"));
    assert!(!valida_cpf("31411965094")); // CPF sem pontos e traços

    // CNPJs aleatórios
    assert!(!valida_cpf("17.578.468/0001-60"));
    assert!(!valida_cpf("26.440.024/0001-55"));
    assert!(!valida_cpf("31.060.467/0001-50"));
    assert!(!valida_cpf("88.216.800/0001-95"));
}

pub fn valida_dados(dados: &ClienteRecv) -> Result<(), String> {
    // Validação de CPF e CNPJ
    if (!dados.pj) && (!valida_cpf(&dados.docto)) {
            return Err(String::from(
                "{ \"mensagem\": \"CPF inválido\" }"
            ));
    } else if dados.pj && (!valida_cnpj(&dados.docto)) {
            return Err(String::from(
                "{ \"mensagem\": \"CNPJ inválido\" }"
            ));
    }

    for e in &dados.enderecos {
        // Validação de unidades federativas conhecidas
        if !ESTADOS.contains(&e.uf.as_str()) {
            return Err(format!(
                "{{ \"mensagem\": \"UF desconhecido: {}\" }}", e.uf
            ))
        }
    }

    Ok(())
}
