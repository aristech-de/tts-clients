import grpc
import re
import wave
import io

from typing import Iterable, cast

from .proto.TTSServices_pb2_grpc import *
from .proto.TTSServices_pb2 import *
from .proto.TTSTypes_pb2 import *

class TtsClient:
    host: str
    ssl: bool
    rootCert: bytes
    auth_token: str
    auth_secret: str
    channel: grpc.Channel

    def __init__(self, host: str, ssl=None, root_cert=b"", auth_token="", auth_secret=""):
      """
      Initializes the client with the given connection parameters.

      Args:
        host (str): The host to connect to. Can include the port, e.g. "localhost:9424".
        ssl (bool): Whether to use SSL. If not explicitly set, the client will try to guess based on the remaining parameters.
        root_cert (bytes): The root certificate to use for SSL (e.g. when connecting to a server that uses a self-signed certificate).
        auth_token (str): The auth token to use for authentication.
        auth_secret (str): The auth secret to use for authentication.
      """
      # When ssl or rootCert are not explicitly set, we check if the host includes the port 9424 or 9423.
      # If host does not include the port, we assume ssl is True and the port is 9424 therefore.
      defaultSsl = (ssl is None and len(root_cert) == 0) or (ssl is True or len(root_cert) != 0)
      (h, p) = self._get_host_port(host, defaultSsl)
      self.host = h + ":" + p
      self.ssl = ssl is True or len(root_cert) != 0 or p == "9424"
      self.rootCert = root_cert
      self.auth_token = auth_token
      self.auth_secret = auth_secret
      if self.ssl or self.rootCert:
        self.channel = self._create_secure_channel()
      else:
        self.channel = grpc.insecure_channel(self.host)
    
    def _get_host_port(self, host, defaultSsl):
      portRe = r"^(?P<host>[^:]+):(?P<port>[0-9]+)$"
      matches = re.search(portRe, host)
      defaultPort = defaultSsl and "9424" or "9423"
      return (host, defaultPort) if matches is None else (matches.group("host"), matches.group("port"))
    
    def _metadata_callback(self, context, callback):
      callback([('token', self.auth_token), ('secret', self.auth_secret)], None)

    def _create_secure_channel(self):
        if len(self.rootCert) != 0:
          cert_creds = grpc.ssl_channel_credentials(root_certificates=self.rootCert)
        else:
          cert_creds = grpc.ssl_channel_credentials()
        auth_creds = grpc.metadata_call_credentials(self._metadata_callback)
        combined_creds = grpc.composite_channel_credentials(cert_creds, auth_creds)
        channel = grpc.secure_channel(target=self.host, credentials=combined_creds)
        return channel

    def list_voices(self, request=VoiceListRequest()) -> Iterable[Voice]:
      """
      Lists the available voices.

      Args:
        request (VoiceListRequest): The request to send. Defaults to an empty request.
      
      Returns:
        VoiceListResponse: The response received.
      """
      stub = SpeechServiceStub(self.channel)
      return stub.GetVoiceList(request)
    
    def get_transcription(self, request: TranscriptionRequest) -> TranscriptionResponse:
      """
      Gets the transcription for the given audio file.

      Args:
        request (TranscriptionRequest): The request to send.
      
      Returns:
        TranscriptionResponse: The response received.
      """
      stub = SpeechServiceStub(self.channel)
      return stub.GetTranscription(request)
    
    def get_phoneset(self, request: PhonesetRequest) -> PhonesetResponse:
      """
      Gets the phoneset for the given voice.

      Args:
        request (PhonesetRequest): The request to send.
      
      Returns:
        PhonesetResponse: The response received.
      """
      stub = SpeechServiceStub(self.channel)
      return stub.GetPhoneset(request)
    
    def synthesize(self, request: SpeechRequest) -> bytes:
      """
      Synthesizes the given text.

      Args:
        request (SpeechRequest): The request to send.
      
      Returns:
        bytes: The audio data received. If the output format is wave, the header is included.
      """
      stream, voice = self.stream_audio(request)
      
      # If the requested output format is wave, we use the wave module to create a wave file with correct header.
      if request.options.audio.container == SpeechAudioFormat.WAV:
        buffer = io.BytesIO()
        wave_file = wave.open(buffer, "wb")
        wave_file.setnchannels(voice.audio.channels)
        wave_file.setsampwidth(voice.audio.bitrate // 8)
        wave_file.setframerate(voice.audio.samplerate)
        for chunk in stream:
          wave_file.writeframes(chunk.data)
        wave_file.close()
        buffer.seek(0)
        return buffer.getvalue()
      
      # Otherwise we return the raw audio data
      audio_data = b""
      for chunk in stream:
        audio_data += chunk.data
      return audio_data
    
    def stream_audio(self, request: SpeechRequest) -> tuple[Iterable[SpeechResponse], Voice]:
      """
      Streams the audio data for the given text.

      Args:
        request (SpeechRequest): The request to send.
      
      Returns:
        tuple[Iterable[SpeechResponse], Voice]: The audio data and the voice used for the request.
      """
      stub = SpeechServiceStub(self.channel)
      voices = self.list_voices(VoiceListRequest())
      voice = next((v for v in voices if v.voice_id == request.options.voice_id), None)
      if voice is None:
        raise ValueError(f"Voice with id {request.options.voice_id} not found")
      return stub.GetSpeech(request), voice