mod utils;
use std::{error::Error, io::Write};
use tonic::codec::CompressionEncoding;
use utils::get_tls_options;

use aristech_tts_client::{
    get_client, get_voices,
    tts_services::{SpeechRequest, SpeechRequestOption},
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

    // Get the voices native audio format to be able
    // to start a sox play command with the correct format
    let voice_id = std::env::var("VOICE_ID").unwrap_or("anne_en_GB".to_string());
    let voices = get_voices(&mut client, None).await?;
    let voice = voices
        .iter()
        .find(|v| v.voice_id == voice_id)
        .expect("Voice not found");

    // Start a sox play command with the correct format
    let voice_audio_spec = voice.audio.unwrap_or_default();
    // Get OSString from the voice audio spec
    let bytes_per_sample = voice_audio_spec.bitrate.to_string();
    let sample_rate = voice_audio_spec.samplerate.to_string();
    let channels = voice_audio_spec.channels.to_string();
    let mut sox = std::process::Command::new("play")
        .arg("-q")
        .arg("-t")
        .arg("raw")
        .arg("-e")
        .arg("signed-integer")
        .arg("-b")
        .arg(bytes_per_sample)
        .arg("-r")
        .arg(sample_rate)
        .arg("-c")
        .arg(channels)
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start sox play command");

    let request = SpeechRequest {
      text: "Thanks for choosing Aristech. For more information about our products visit us at aristech.de".to_string(),
      options: Some(SpeechRequestOption {
        voice_id: std::env::var("VOICE_ID").unwrap_or("anne_en_GB".to_string()),
        ..SpeechRequestOption::default()
      }),
      ..SpeechRequest::default()
    };
    let mut stream = client.get_speech(request).await?.into_inner();
    let sox_stdin = sox.stdin.as_mut().expect("Failed to open sox stdin");
    while let Some(response) = stream.message().await? {
        sox_stdin
            .write_all(&response.data)
            .expect("Failed to write to sox stdin");
    }

    Ok(())
}
