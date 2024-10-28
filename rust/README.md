# Aristech TTS-Client for Rust

This is the Rust client implementation for the Aristech TTS-Server.

## Installation

To use the client in your project, add it to your `Cargo.toml` or use `cargo` to add it:

```sh
cargo add aristech-tts-client
```

## Usage

```rust
use aristech_tts_client::{get_client, synthesize, SpeechRequest, SpeechRequestOption, TlsOptions};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = get_client(
        "https://tts.example.com",
        Some(TlsOptions::default()),
    ).await?;

    let request = SpeechRequest {
        text: "Text to speak.".to_string(),
        options: Some(SpeechRequestOption {
            voice_id: "anne_en_GB".to_string(),
          ..SpeechRequestOption::default()
        }),
        ..SpeechRequest::default()
    };
    let data = synthesize(&mut client, request).await?;
    std::fs::write("output.wav", data).expect("Unable to write file");

    Ok(())
}
```

There are several examples in the [examples](.) directory:

- [file.rs](examples/file.rs): Demonstrates how convert text to speech and save the audio to a file.
- [streaming.rs](examples/streaming.rs): Demonstrates how to stream audio to a sox process which plays the audio as it is being streamed.
- [voices.rs](examples/voices.rs): Demonstrates how to get the available voices from the server.
- [phoneset.rs](examples/phoneset.rs): Demonstrates how to retrieve the phoneset for a voice.
- [transcribe.rs](examples/transcribe.rs): Demonstrates how to retrieve the pronunciation of a word for a voice.

You can run the examples directly using `cargo` like this:

1. Create a `.env` file in the [rust](.) directory:

```sh
HOST=https://tts.example.com # Note: The protocol is required in the rust client
# The credentials are optional but probably required for most servers:
TOKEN=your-token
SECRET=your-secret

# The following are optional:
# ROOT_CERT=your-root-cert.pem # If the server uses a self-signed certificate
# SSL=true # Set to true if credentials are provided or if a ROOT_CERT is provided
# VOICE_ID=some-available-voice-id
```

2. Run the examples, e.g.:

```sh
cargo run --example file
```

## Build

To build the library, run:

```bash
cargo build
```