mod utils;
use std::error::Error;
use tonic::codec::CompressionEncoding;
use utils::get_tls_options;

use aristech_tts_client::{
    get_client, synthesize,
    tts_services::{
        speech_audio_format::{Codec, Container},
        SpeechAudioFormat, SpeechRequest, SpeechRequestOption,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options)
        .await?
        .accept_compressed(CompressionEncoding::Gzip);
    let request = SpeechRequest {
      text: "Thanks for choosing Aristech. For more information about our products visit us at aristech.de".to_string(),
      options: Some(SpeechRequestOption {
        voice_id: std::env::var("VOICE_ID").unwrap_or("anne_en_GB".to_string()),
        audio: Some(
          SpeechAudioFormat {
            container: Container::Wav as i32,
            codec: Codec::Pcm as i32,
            ..Default::default()
          }
        ),
        ..Default::default()
      }),
      ..Default::default()
    };
    let data = synthesize(&mut client, request).await?;
    std::fs::write("output.wav", data).expect("Unable to write file");

    Ok(())
}
