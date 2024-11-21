//! # Aristech TTS-Client
//! The Aristech TTS-Client is a client library for the Aristech TTS-Server.

#![warn(missing_docs)]

/// The tts_services module contains types and functions generated from the Aristech TTS proto files.
pub mod tts_services {
    #![allow(missing_docs)]
    tonic::include_proto!("aristech.tts");
}

use tts_services::speech_audio_format::Container;
use tts_services::speech_service_client::SpeechServiceClient as TtsServiceClient;
use tts_services::{
    PhonesetRequest, PhonesetResponse, SpeechRequest, TranscriptionRequest, TranscriptionResponse,
    Voice, VoiceListRequest,
};

use std::error::Error;
use std::io::Cursor;

use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

/// The Auth struct holds the token and secret needed to authenticate with the server.
#[derive(Clone)]
pub struct Auth {
    /// The token to authenticate with the server
    pub token: String,
    /// The secret to authenticate with the server
    pub secret: String,
}

impl Auth {
    /// Creates a new Auth struct with the given token and secret.
    pub fn new(token: &str, secret: &str) -> Self {
        Self {
            token: token.to_string(),
            secret: secret.to_string(),
        }
    }
}

/// The AuthInterceptor struct is used to intercept requests to the server and add the authentication headers.
pub struct AuthInterceptor {
    /// The authentication data to add to the headers
    auth: Option<Auth>,
}

impl AuthInterceptor {
    /// Creates a new AuthInterceptor with the given authentication data.
    fn new(auth: Option<Auth>) -> Self {
        Self { auth }
    }
}

impl Interceptor for AuthInterceptor {
    /// Adds the authentication data to the headers of the request.
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        if let Some(auth) = &self.auth {
            let mut request = request;
            request
                .metadata_mut()
                .insert("token", auth.token.parse().unwrap());
            request
                .metadata_mut()
                .insert("secret", auth.secret.parse().unwrap());
            Ok(request)
        } else {
            Ok(request)
        }
    }
}

/// The TtsClient type is a type alias for the TtsServiceClient with the AuthInterceptor.
pub type TtsClient = TtsServiceClient<InterceptedService<Channel, AuthInterceptor>>;

/// The TlsOptions struct holds the tls options needed to communicate with the server.
#[derive(Clone, Default)]
pub struct TlsOptions {
    /// The authentication data to authenticate with the server
    pub auth: Option<Auth>,
    /// The root certificate to verify the server's certificate
    /// This is usually only needed when the server uses a self-signed certificate
    pub ca_certificate: Option<String>,
}

impl TlsOptions {
    /// Creates a new TlsOptions struct with the given authentication data and root certificate.
    pub fn new(auth: Option<Auth>, ca_certificate: Option<String>) -> Self {
        Self {
            auth,
            ca_certificate,
        }
    }
}

/// Creates a new [TtsClient] to communicate with the server.
///
/// # Arguments
/// * `host` - The host to connect to (might include the port number e.g. "https://tts.example.com:8424"). Note that the protocol must be included in the host.
/// * `tls_options` - The tls options to use when connecting to the server. If None is given, the connection will be unencrypted and unauthenticated (the server must also be configured to communicate without encryption in this case).
pub async fn get_client(
    host: String,
    tls_options: Option<TlsOptions>,
) -> Result<TtsClient, Box<dyn Error>> {
    // Check if a schema is included in the host
    // otherwise add http if no tls options are given and https otherwise
    let host = if host.starts_with("http://") || host.starts_with("https://") {
        host
    } else {
        match tls_options {
            Some(_) => format!("https://{}", host),
            None => format!("http://{}", host),
        }
    };
    match tls_options {
        Some(tls_options) => {
            let tls = match tls_options.ca_certificate {
                Some(ca_certificate) => {
                    let ca_certificate = Certificate::from_pem(ca_certificate);
                    ClientTlsConfig::new().ca_certificate(ca_certificate)
                }
                None => ClientTlsConfig::with_native_roots(ClientTlsConfig::new()),
            };
            let channel = Channel::from_shared(host)?
                .tls_config(tls)?
                .connect()
                .await?;
            let client: TtsServiceClient<InterceptedService<Channel, AuthInterceptor>> =
                TtsServiceClient::with_interceptor(channel, AuthInterceptor::new(tls_options.auth));
            Ok(client)
        }
        None => {
            let channel = Channel::from_shared(host)?.connect().await?;
            let client: TtsServiceClient<InterceptedService<Channel, AuthInterceptor>> =
                TtsServiceClient::with_interceptor(channel, AuthInterceptor::new(None));
            Ok(client)
        }
    }
}

/// Gets the list of available voices from the server.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server. If None is given, the default request will be used.
///
/// # Example
///
/// ```no_run
/// use aristech_tts_client::{get_client, TlsOptions, get_voices};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://tts.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let voices = get_voices(&mut client, None).await?;
///     for voice in voices {
///         println!("Voice: {:?}", voice);
///     }
///    Ok(())
/// }
/// ```
pub async fn get_voices(
    client: &mut TtsClient,
    request: Option<VoiceListRequest>,
) -> Result<Vec<Voice>, Box<dyn Error>> {
    let req = request.unwrap_or(VoiceListRequest::default());
    let request = tonic::Request::new(req);
    let response = client.get_voice_list(request).await?;
    let mut stream = response.into_inner();
    let mut voices = vec![];
    while let Ok(Some(voice)) = stream.message().await {
        voices.push(voice);
    }
    Ok(voices)
}

/// Gets the phoneset for the given voice from the server.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server.
///
/// # Example
///
/// ```no_run
/// use aristech_tts_client::{
///     get_client, TlsOptions, get_phoneset,
///     tts_services::{PhonesetRequest, Voice},
/// };
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://tts.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let response = get_phoneset(&mut client, PhonesetRequest {
///         voice: Some(Voice {
///             voice_id: "anne_en_GB".to_string(),
///             ..Voice::default()
///         }),
///         ..PhonesetRequest::default()
///     }).await?;
///     println!("{:?}", response);
///     Ok(())
/// }
pub async fn get_phoneset(
    client: &mut TtsClient,
    request: PhonesetRequest,
) -> Result<PhonesetResponse, Box<dyn Error>> {
    let request = tonic::Request::new(request);
    let response = client.get_phoneset(request).await?;
    Ok(response.into_inner())
}

/// Gets the transcription for a word for a specific voice from the server.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server.
///
/// # Example
///
/// ```no_run
/// use aristech_tts_client::{
///    get_client, TlsOptions, get_transcription,
///   tts_services::{TranscriptionRequest, Voice},
/// };
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://tts.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let response = get_transcription(&mut client, TranscriptionRequest {
///         voice: Some(Voice {
///             voice_id: "anne_en_GB".to_string(),
///             ..Voice::default()
///         }),
///         word: "hello".to_string(),
///         ..TranscriptionRequest::default()
///     }).await?;
///     println!("{:?}", response);
///     Ok(())
/// }
/// ```
///
pub async fn get_transcription(
    client: &mut TtsClient,
    request: TranscriptionRequest,
) -> Result<TranscriptionResponse, Box<dyn Error>> {
    let request = tonic::Request::new(request);
    let response = client.get_transcription(request).await?;
    Ok(response.into_inner())
}

/// Synthesizes the given text with the given options and returns the audio data.
/// Currently only raw and wav audio formats are supported.
/// If the audio format is set to wav (default), the audio data will be prepended with a wave header.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server.
///
/// # Example
///
/// ```no_run
/// use aristech_tts_client::{
///     get_client, TlsOptions, synthesize,
///     tts_services::{SpeechRequest, SpeechRequestOption},
/// };
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://tts.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let request = SpeechRequest {
///         text: "Hello world.".to_string(),
///         options: Some(SpeechRequestOption {
///             voice_id: "anne_en_GB".to_string(),
///             ..SpeechRequestOption::default()
///         }),
///         ..SpeechRequest::default()
///     };
///     let data = synthesize(&mut client, request).await?;
///     std::fs::write("output.wav", data)?;
///     Ok(())
/// }
pub async fn synthesize(
    client: &mut TtsClient,
    request: SpeechRequest,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Get the voice list to check if the voice exists
    // and be able to create a header with the voices audio format
    let voices = get_voices(client, None).await?;
    let voice = voices
        .iter()
        .find(|voice| voice.voice_id == request.clone().options.unwrap_or_default().voice_id)
        .ok_or("Voice not found")?;

    // Start the audio stream
    let req = request.clone();
    let request = tonic::Request::new(request);
    let response = client.get_speech(request).await?;
    let mut stream = response.into_inner();
    let mut audio = vec![];
    while let Ok(Some(chunk)) = stream.message().await {
        // audio.extend_from_slice(&chunk.data);
        audio.extend_from_slice(&chunk.data);
    }

    // Check which audio format was requested
    let audio_spec = req.options.unwrap_or_default().audio.unwrap_or_default();
    match Container::try_from(audio_spec.container)? {
        Container::Wav => {
            let voice_audio_spec = voice.audio.unwrap_or_default();
            // The server streams audio data as it is generated
            // therefore and because wave headers need to know the size of the audio data
            // we need to create a wave header and prepend it to the audio data ourselves
            let spec = hound::WavSpec {
                channels: voice_audio_spec.channels as u16,
                sample_rate: voice_audio_spec.samplerate as u32,
                bits_per_sample: voice_audio_spec.bitrate as u16,
                sample_format: hound::SampleFormat::Int,
            };
            // Create a wave header as vec of bytes
            let mut wav = Cursor::new(vec![]);
            let mut writer = hound::WavWriter::new(&mut wav, spec)?;
            // Write the audio data to the wave
            for sample in audio.chunks(voice_audio_spec.bitrate as usize / 8) {
                for i in 0..voice_audio_spec.channels {
                    writer.write_sample(i16::from_le_bytes([
                        sample[i as usize * 2],
                        sample[i as usize * 2 + 1],
                    ]))?;
                }
            }
            // Get the audio data from writer
            writer.finalize()?;
            let audio = wav.into_inner();
            Ok(audio)
        }
        _ => Ok(audio),
    }
}

/// This is just an alias for [synthesize]
pub async fn get_audio(
    client: &mut TtsClient,
    request: SpeechRequest,
) -> Result<Vec<u8>, Box<dyn Error>> {
    synthesize(client, request).await
}
