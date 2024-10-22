mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_tts_client::{
    get_client, get_phoneset,
    tts_services::{PhonesetRequest, Voice},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let request = PhonesetRequest {
        voice: Some(Voice {
            voice_id: std::env::var("VOICE_ID").unwrap_or("anne_en_GB".to_string()),
            ..Voice::default()
        }),
        ..PhonesetRequest::default()
    };

    let phoneset = get_phoneset(&mut client, request).await?;
    println!("{:?}", phoneset.phoneset);
    Ok(())
}
