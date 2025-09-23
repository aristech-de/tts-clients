import sys

from aristech_tts_client import TtsClient, SpeechRequest, SpeechRequestOption, SpeechAudioFormat
from utils import host, auth_token, auth_secret, root_cert, ssl, voice_id

# Use the first argument or a default text
text = sys.argv[1] if len(sys.argv) > 1 else "Thanks for choosing Aristech. For more information about our products visit us at aristech.de"

client = TtsClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)
data = client.synthesize(SpeechRequest(
    text=text,
    options=SpeechRequestOption(
        voice_id=voice_id,
        audio=SpeechAudioFormat(
            container=SpeechAudioFormat.Container.WAV,
            codec=SpeechAudioFormat.Codec.PCM
        )
    )
))
# Write the audio data to a file
with open("output.wav", "wb") as f:
    f.write(data)