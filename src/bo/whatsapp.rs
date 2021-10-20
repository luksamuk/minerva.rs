// bo/whatsapp.rs -- Uma parte de Minerva.rs
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
use s3::{
    bucket::Bucket,
    creds::Credentials,
};

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
/// A funcionalidade opera através do serviço Twilio, que pode ser testado
/// através de uma Sandbox para WhatsApp, caso não exista um número Twilio
/// contratado para produção.
/// 
/// Este procedimento espera que as variáveis de ambiente a seguir estejam
/// apropriadamente definidas:
/// - `TWILIO_SID` (SID da conta Twilio)
/// - `TWILIO_TOKEN` (Token secreto de acesso do Twilio)
/// - `TWILIO_PHONE` (Telefone do remetente da mensagem)
/// - `TWILIO_CLIENT_NUMBER` (Telefone do destinatário da mensagem)
///
/// # Formato dos telefones
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

/// Envia uma mensagem de texto com mídia anexada, através do Sandbox do Twilio
/// para WhatsApp.
///
/// A funcionalidade opera através do serviço Twilio, que pode ser testado
/// através de uma Sandbox para WhatsApp, caso não exista um número Twilio
/// contratado para produção.
/// 
/// Arquivos enviados são armazenados em um bucket do serviço AWS S3, uma vez
/// que o Twilio requer o URL da mídia a ser anexada. Isso requer que o recurso
/// seja acessível para o Twilio. Essa configuração pode ser alcançada definindo
/// a política de acesso dos objetos no bucket como pública, ou através de
/// configuração fina para garantir acesso do Twilio ao recurso no S3.
/// 
/// Caso o arquivo apontado seja um documento, o texto não será mostrado na
/// mensagem enviada.
/// 
/// Este procedimento espera que as variáveis de ambiente a seguir estejam
/// apropriadamente definidas:
/// - `TWILIO_SID` (SID da conta Twilio)
/// - `TWILIO_TOKEN` (Token secreto de acesso do Twilio)
/// - `TWILIO_PHONE` (Telefone do remetente da mensagem)
/// - `TWILIO_CLIENT_NUMBER` (Telefone do destinatário da mensagem)
/// - `AWS_S3_BUCKET` (Nome do bucket do AWS S3)
/// - `AWS_S3_REGION` (Região onde o bucket AWS S3 foi criado)
/// - `AWS_S3_ACCESS_KEY` (Chave de acesso do AWS S3)
/// - `AWS_S3_ACCESS_SECRET` (Token secreto de acesso do AWS S3)
/// 
/// ## Formato dos telefones
/// Os números de telefone devem ser fornecidos no formato `+WWXXYYYYYYYYY` onde
/// `WW` é o código do país, `XX` é o DDD e `YYYYYYYYY` são os números do
/// telefone. Por exemplo, o número brasileiro `(11) 98999-9999` seria
/// representado como `+5511989999999`.
/// 
/// ## Bugs conhecidos
/// Só serão enviados arquivos que estão na pasta de execução deste projeto,
/// informados via `nome_arquivo`. Esta limitação arbitrária dá-se pela natureza
/// de teste dessa função.
/// 
/// Os arquivos enviados para o bucket AWS S3 não são gerenciados, sendo
/// armazenados indefinidamente. Dada a natureza, mais uma vez, de teste desta
/// função, será necessário limpar os objetos no bucket manualmente.
pub fn envia_arquivo_whatsapp_sandbox(mensagem: String, nome_arquivo: &'static str) {
    let sid = env::var("TWILIO_SID").unwrap();
    let token = env::var("TWILIO_TOKEN").unwrap();

    let sandbox_number = env::var("TWILIO_PHONE")
        .expect("Defina a variável TWILIO_PHONE");
    let client_number = env::var("TWILIO_CLIENT_NUMBER")
        .expect("Defina a variável TWILIO_CLIENT_NUMBER");
    
    let bucket_name = env::var("AWS_S3_BUCKET").unwrap();
    let region_name = env::var("AWS_S3_REGION").unwrap();
    let credentials = Credentials::from_env_specific(
        Some("AWS_S3_ACCESS_KEY"),
        Some("AWS_S3_ACCESS_SECRET"),
        None,
        None,
    ).unwrap();

    tokio::spawn(async move {
        use std::fs::File;
        use std::io::{Read, BufReader};

        let region = region_name.parse().unwrap();
        let bucket = Bucket::new(&bucket_name, region, credentials).unwrap();

        // Lê os dados do arquivo
        let f = File::open(nome_arquivo).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        // Envia arquivo para o AWS S3 put_object_blocking
        let (_, return_code) = bucket.put_object(&format!("/{}", nome_arquivo), &buffer)
            .await
            .unwrap();
        
        if return_code != 200 {
            println!("Erro ao enviar a imagem: {}", return_code);
            return;
        }

        // Envia arquivo via whatsapp
        let media_link = format!("https://{}.s3.{}.amazonaws.com/{}",
            bucket_name,
            region_name,
            nome_arquivo,
        );

        let _ = Twilio::new(sid, token).unwrap()
        .send_msg(
            &("whatsapp:".to_owned() + &sandbox_number),
            &("whatsapp:".to_owned() + &client_number),
            &mensagem,
        )
        .media(&media_link)
        .run()
        .await;
        
        // Deleta arquivo do AWS S3
        //let _ = bucket.delete_object(&format!("/{}", nome_arquivo)).await.unwrap();
    });
}