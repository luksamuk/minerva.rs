// bo/twilio.rs -- Uma parte de Minerva.rs
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

//! Este módulo contém ferramentas relacionadas ao envio e recebimento de
//! mensagens através de WhatsApp.

use std::env;
use anyhow::Result;
use twilio_async::Twilio;

/// Cria uma conexão com o serviço Twilio.
/// 
/// Este procedimento espera que as variáveis de ambiente `TWILIO_SID` e
/// `TWILIO_TOKEN` estejam apropriadamente definidas.
pub fn cria_conexao_twilio() -> Result<Twilio> {
    let sid = env::var("TWILIO_SID")?; 
    let token = env::var("TWILIO_TOKEN")?;
    Ok(Twilio::new(sid, token)?)
}
