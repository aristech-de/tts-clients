pub mod tts_services {
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

#[derive(Clone)]
pub struct Auth {
    pub token: String,
    pub secret: String,
}

impl Auth {
    pub fn new(token: &str, secret: &str) -> Self {
        Self {
            token: token.to_string(),
            secret: secret.to_string(),
        }
    }
}

pub struct AuthInterceptor {
    auth: Option<Auth>,
}

impl AuthInterceptor {
    fn new(auth: Option<Auth>) -> Self {
        Self { auth }
    }
}
impl Interceptor for AuthInterceptor {
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

pub type TtsClient = TtsServiceClient<InterceptedService<Channel, AuthInterceptor>>;

#[derive(Clone)]
pub struct TlsOptions {
    pub auth: Option<Auth>,
    pub ca_certificate: Option<String>,
}

impl TlsOptions {
    pub fn new(auth: Option<Auth>, ca_certificate: Option<String>) -> Self {
        Self {
            auth,
            ca_certificate,
        }
    }
}

pub async fn get_client(
    host: String,
    tls_options: Option<TlsOptions>,
) -> Result<TtsClient, Box<dyn Error>> {
    match tls_options {
        Some(tls_options) => {
            let tls = match tls_options.ca_certificate {
                Some(ca_certificate) => {
                    let ca_certificate = Certificate::from_pem(ca_certificate);
                    ClientTlsConfig::new().ca_certificate(ca_certificate)
                    // .domain_name("localhost".to_string())
                }
                None => ClientTlsConfig::new(),
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

pub async fn get_phoneset(
    client: &mut TtsClient,
    request: PhonesetRequest,
) -> Result<PhonesetResponse, Box<dyn Error>> {
    let request = tonic::Request::new(request);
    let response = client.get_phoneset(request).await?;
    Ok(response.into_inner())
}

pub async fn get_transcription(
    client: &mut TtsClient,
    request: TranscriptionRequest,
) -> Result<TranscriptionResponse, Box<dyn Error>> {
    let request = tonic::Request::new(request);
    let response = client.get_transcription(request).await?;
    Ok(response.into_inner())
}

pub async fn get_audio(
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
