// bo/usuarios.rs -- Uma parte de Minerva.rs
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

//! Este módulo contém ferramentas para reforçar regras de negócio relacionadas
//! à validação de transações envolvendo dados de usuários do sistema.

use sodiumoxide::crypto::pwhash::argon2id13;

/// Compara por igualdade uma senha fornecida em texto e a senha de um usuário,
/// utilizando o hash da senha do usuário referido.
/// 
/// A senha deverá fornecida como um slice de String, e o hash da senha deverá
/// ser uma referência direta aos bytes da senha com hash, da forma armazenada
/// no banco de dados, envolvendo o hash e o salt para aquele usuário.
pub fn verifica_senha(senha: &str, senha_hash: &[u8]) -> bool {
    sodiumoxide::init().unwrap();
    match argon2id13::HashedPassword::from_slice(senha_hash) {
        Some(hp) => argon2id13::pwhash_verify(&hp, senha.as_bytes()),
        None => false,
    }
}

/// Gera um hash de senha a partir dos bytes diretos de uma senha em texto.
/// 
/// O argumento `senha_bytes` deverá ser o texto da senha como uma referência
/// direta à mesma, na forma de um vetor de bytes individuais. Será retornado
/// o hash da senha fornecida, igualmente na forma de um array de bytes, que
/// poderá ser copiada para um Vec ou estrutura similar.
/// 
/// Será utilizado o algoritmo Argon2, mais especificamente uma solução híbrida
/// de Argon2i (resistente a ataques de _side-channel_) e Argon2d (resistente a
/// ataques de _time-memory tradeoff (TMTO)_).
/// 
/// Por limitações da `libsodium`, a função Argon2 possui um salt fixo de 128
/// bits e um parâmetro de paralelismo fixo em `1`.
/// 
/// A geração do hash pode ser intensivo em termos de CPU e memória. Por isso,
/// são utilizado os parâmetros recomendados em [`sodiumoxide`], que são valores
/// relativamente seguros para tal cálculo.
/// 
/// O hash retornado compreende três valores que poderão ser diretamente salvos
/// no banco de dados para teste posterior, sendo eles:
/// 
/// - O resultado esperado da aplicação da função hash;
/// - O _salt_ gerado automaticamente para a aplicação da função;
/// - Os parâmetros `OPSLIMIT` e `MEMLIMIT` empregados no cálculo, que serão
///   necessários na verificação da senha a partir de tal hash.
pub fn gera_hash_senha(senha_bytes: &[u8]) -> argon2id13::HashedPassword {
    sodiumoxide::init().unwrap();
    argon2id13::pwhash(
        senha_bytes,
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap()
}