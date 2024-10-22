mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_tts_client::{get_client, get_voices};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let voices = get_voices(&mut client, None).await?;
    for voice in voices {
        println!("{:?}", voice);
    }
    Ok(())
}
