from aristech_tts_client import TtsClient
from utils import host, auth_token, auth_secret, root_cert, ssl

client = TtsClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)
voices = client.list_voices()
for voice in voices:
    print(voice)
