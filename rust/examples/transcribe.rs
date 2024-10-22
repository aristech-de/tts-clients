mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_tts_client::{
    get_client, get_transcription,
    tts_services::{TranscriptionRequest, Voice},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let request = TranscriptionRequest {
        voice: Some(Voice {
            voice_id: std::env::var("VOICE_ID").unwrap_or("anne_en_GB".to_string()),
            ..Voice::default()
        }),
        word: "hello".to_string(),
        ..TranscriptionRequest::default()
    };

    let response = get_transcription(&mut client, request).await?;
    println!("{:?}", response.transcription);
    Ok(())
}
