//! # Aristech TTS-Client
//! The Aristech TTS-Client is a client library for the Aristech TTS-Server.

#![warn(missing_docs)]

/// The tts_services module contains types and functions generated from the Aristech TTS proto files.
pub mod tts_services {
    #![allow(missing_docs)]
    tonic::include_proto!("aristech.tts");
}

use tts_services::speech_service_client::SpeechServiceClient as TtsServiceClient;
use tts_services::{
    PhonesetRequest, PhonesetResponse, SpeechRequest, TranscriptionRequest, TranscriptionResponse,
    Voice, VoiceListRequest,
};

use std::error::Error;
use std::fmt;
use std::time::Duration;

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

/// Options that control how the client connects to the server.
///
/// Create it with [ConnectOptions::new] (or `Default`) and set the options with the builder methods.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ConnectOptions {
    /// The maximum time to wait for the connection, see [ConnectOptions::connect_timeout].
    pub connect_timeout: Option<Duration>,
}

impl ConnectOptions {
    /// Creates connect options without any limits, which is how [get_client] connects.
    pub fn new() -> Self {
        Self::default()
    }

    /// Limits how long establishing the connection may take.
    ///
    /// The limit covers the whole connection setup: the TCP connect and, with TLS, the TLS
    /// handshake. When it is exceeded, connecting fails with a [ConnectTimeoutError].
    /// Without a limit, connecting waits as long as the operating system does, which for an
    /// unreachable host is typically about two minutes and for a TLS server that accepts the TCP
    /// connection but never answers the handshake is indefinitely.
    ///
    /// The limit only applies to establishing the connection. Requests on an established
    /// connection are not limited by it. This includes a plaintext server that accepts the TCP
    /// connection but never answers: the connection counts as established and the first request
    /// waits instead.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }
}

/// The error returned when the connection to the server could not be established within the
/// [connect timeout](ConnectOptions::connect_timeout).
///
/// [get_client_with_options] returns it boxed, use `downcast_ref` to tell it apart from other
/// connection errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectTimeoutError {
    /// The host the client tried to connect to.
    pub host: String,
    /// The connect timeout that was exceeded.
    pub timeout: Duration,
}

impl fmt::Display for ConnectTimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "connect to {}: timed out after {} ms",
            self.host,
            self.timeout.as_millis()
        )
    }
}

impl Error for ConnectTimeoutError {}

/// Walks the error source chain and joins each level with ": ".
fn error_chain(e: &dyn std::error::Error) -> String {
    let mut msg = e.to_string();
    let mut source = e.source();
    while let Some(cause) = source {
        msg.push_str(": ");
        msg.push_str(&cause.to_string());
        source = cause.source();
    }
    msg
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
    get_client_with_options(host, tls_options, ConnectOptions::default()).await
}

/// Creates a new [TtsClient] to communicate with the server, using the given [ConnectOptions].
///
/// # Arguments
/// * `host` - The host to connect to (might include the port number e.g. "https://tts.example.com:8424"). Note that the protocol must be included in the host.
/// * `tls_options` - The tls options to use when connecting to the server. If None is given, the connection will be unencrypted and unauthenticated (the server must also be configured to communicate without encryption in this case).
/// * `options` - Options that control how the connection is established, e.g. a connect timeout.
///
/// # Example
///
/// ```no_run
/// use aristech_tts_client::{get_client_with_options, ConnectOptions, ConnectTimeoutError, TlsOptions};
/// use std::error::Error;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let options = ConnectOptions::new().connect_timeout(Duration::from_secs(5));
///     match get_client_with_options("https://tts.example.com:8424".to_string(), Some(TlsOptions::default()), options).await {
///         Ok(_client) => println!("connected"),
///         Err(e) if e.downcast_ref::<ConnectTimeoutError>().is_some() => println!("server did not answer in time"),
///         Err(e) => return Err(e),
///     }
///     Ok(())
/// }
/// ```
pub async fn get_client_with_options(
    host: String,
    tls_options: Option<TlsOptions>,
    options: ConnectOptions,
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
    let (endpoint, auth) = match tls_options {
        Some(tls_options) => {
            let tls = match tls_options.ca_certificate {
                Some(ca_certificate) => {
                    let ca_certificate = Certificate::from_pem(ca_certificate);
                    ClientTlsConfig::new().ca_certificate(ca_certificate)
                }
                None => ClientTlsConfig::with_native_roots(ClientTlsConfig::new()),
            };
            (
                Channel::from_shared(host.clone())?.tls_config(tls)?,
                tls_options.auth,
            )
        }
        None => (Channel::from_shared(host.clone())?, None),
    };
    // The timeout wraps the whole connect instead of using tonic's `Endpoint::connect_timeout`,
    // which only limits the TCP connect and not the TLS handshake.
    let connected = match options.connect_timeout {
        Some(timeout) => tokio::time::timeout(timeout, endpoint.connect())
            .await
            .map_err(|_| ConnectTimeoutError {
                host: host.clone(),
                timeout,
            })?,
        None => endpoint.connect().await,
    };
    let channel = connected.map_err(|e| format!("connect to {}: {}", host, error_chain(&e)))?;
    let client: TtsServiceClient<InterceptedService<Channel, AuthInterceptor>> =
        TtsServiceClient::with_interceptor(channel, AuthInterceptor::new(auth));
    Ok(client)
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
    // let voice = voices
    voices
        .iter()
        .find(|voice| voice.voice_id == request.clone().options.unwrap_or_default().voice_id)
        .ok_or("Voice not found")?;
    // Start the audio stream
    let request = tonic::Request::new(request);
    let response = client.get_speech(request).await?;
    let mut stream = response.into_inner();
    let mut audio = vec![];
    while let Ok(Some(chunk)) = stream.message().await {
        // audio.extend_from_slice(&chunk.data);
        audio.extend_from_slice(&chunk.data);
    }

    Ok(audio)
}

/// This is just an alias for [synthesize]
pub async fn get_audio(
    client: &mut TtsClient,
    request: SpeechRequest,
) -> Result<Vec<u8>, Box<dyn Error>> {
    synthesize(client, request).await
}

/// Clears the cache of the server, removing all cached audio data.
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server. If None is given, the default request will be used.
/// # Example
/// ```no_run
/// use aristech_tts_client::{get_client, TlsOptions, clear_cache};
/// use std::error::Error;
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://tts.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let response = clear_cache(&mut client, None).await?;
///     println!("Cache cleared: {:?}", response);
///     Ok(())
/// }
/// ```
pub async fn clear_cache(
    client: &mut TtsClient,
    request: Option<tts_services::ClearCacheRequest>,
) -> Result<tts_services::ClearCacheResponse, Box<dyn Error>> {
    let req = request.unwrap_or(tts_services::ClearCacheRequest::default());
    let request = tonic::Request::new(req);
    let response = client.clear_cache(request).await?;
    Ok(response.into_inner())
}
