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
use twilio_async::{Twilio, TwilioRequest};

/// Cria uma conexão com o serviço Twilio.
/// 
/// Este procedimento espera que as variáveis de ambiente `TWILIO_SID` e
/// `TWILIO_TOKEN` estejam apropriadamente definidas.
pub fn cria_conexao_twilio() -> Result<Twilio> {
    let sid = env::var("TWILIO_SID")?; 
    let token = env::var("TWILIO_TOKEN")?;
    Ok(Twilio::new(sid, token)?)
}

/// Envia uma mensagem de texto através do Sandbox do Twilio para WhatsApp.
///
/// Este procedimento espera que as variáveis de ambiente `TWILIO_SID` e
/// `TWILIO_TOKEN` estejam apropriadamente definidas.
/// 
/// A função também opera com o pressuposto de que exista um número para o
/// serviço configurado na variável de ambiente `TWILIO_PHONE`, e também que o
/// número do destinatário esteja configurado na variável `TWILIO_CLIENT_NUMBER`.
///
/// Os números de telefone devem ser fornecidos no formato `+WWXXYYYYYYYYY` onde
/// `WW` é o código do país, `XX` é o DDD e `YYYYYYYYY` são os números do
/// telefone. Por exemplo, o número brasileiro `(11) 98999-9999` seria
/// representado como `+5511989999999`.
pub fn envia_mensagem_whatsapp_sandbox(mensagem: String) {
    let sid = env::var("TWILIO_SID").unwrap();
    let token = env::var("TWILIO_TOKEN").unwrap();
    
    let sandbox_number = env::var("TWILIO_PHONE")
        .expect("Defina a variável TWILIO_PHONE");
    let client_number = env::var("TWILIO_CLIENT_NUMBER")
        .expect("Defina a variável TWILIO_CLIENT_NUMBER");

    tokio::spawn(async move {
        let _ = Twilio::new(sid, token).unwrap().send_msg(
            &("whatsapp:".to_owned() + &sandbox_number),
            &("whatsapp:".to_owned() + &client_number),
            &mensagem,
        ).run().await;
    });
}
