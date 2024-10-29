# Aristech TTS-Client for Python

This is the Python client implementation for the Aristech TTS-Server.

## Installation

```bash
pip install aristech-tts-client
```

## Usage

```python
from aristech_tts_client import TtsClient

client = TtsClient(host='tts.example.com')
results = client.recognize_file("path/to/audio/file.wav", RecognitionConfig(specification=RecognitionSpec(model="some-model")))
print('\n'.join([r.chunks[0].alternatives[0].text for r in results]))
```

There are several examples in the [examples](.) directory:

- [file.py](./examples/file.py): Pretty much the same as the example above.
- [streaming.py](./examples/streaming.py): Demonstrates how to stream audio to a sox process which plays the audio as it is being streamed.
- [voices.py](./examples/voices.py): Demonstrates how to get the available voices from the server.
- [phoneset.py](examples/phoneset.py): Demonstrates how to get the phoneset for a voice.
- [transcribe.py](examples/transcribe.py): Demonstrates how to get how a voice would pronounce a given word.

You can run the examples directly using `python` like this:

1. Create a `.env` file in the [python](.) directory:

```sh
HOST=tts.example.com
# The credentials are optional but probably required for most servers:
TOKEN=your-token
SECRET=your-secret

# The following are optional:
# ROOT_CERT=your-root-cert.pem # If the server uses a self-signed certificate
# VOICE_ID=some-available-voice-id
```

2. Run the examples, e.g.:

```sh
python examples/streaming.py
```
