import sys
import subprocess

from aristech_tts_client import TtsClient, SpeechRequest, SpeechRequestOption
from utils import host, auth_token, auth_secret, root_cert, ssl, voice_id

# Use the first argument or a default text
text = sys.argv[1] if len(sys.argv) > 1 else "Thanks for choosing Aristech. For more information about our products visit us at aristech.de"

client = TtsClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)
stream, voice = client.stream_audio(SpeechRequest(
    text=text,
    options=SpeechRequestOption(
        voice_id=voice_id
    )
))

# Open a sox player to play the audio stream
audio = voice.audio
player = subprocess.Popen([
    "play", "-t", "raw", "-r", str(audio.samplerate), "-e", "signed", "-b", str(audio.bitrate), "-c", str(audio.channels), "-"],
    stdin=subprocess.PIPE,
    stdout=subprocess.DEVNULL,
    stderr=subprocess.DEVNULL
)

for chunk in stream:
    player.stdin.write(chunk.data)

