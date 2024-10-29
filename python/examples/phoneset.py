import json

from aristech_tts_client import TtsClient, PhonesetRequest, Voice
from utils import host, auth_token, auth_secret, root_cert, ssl, voice_id

client = TtsClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)
response = client.get_phoneset(PhonesetRequest(voice=Voice(voice_id=voice_id)))
print(response.phoneset)