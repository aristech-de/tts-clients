# Aristech TTS-Client for NodeJS

This is the NodeJS client implementation for the Aristech TTS-Server.

## Installation

```bash
npm install @aristech-org/tts-client
```

## Usage

```typescript
import { TtsClient } from '@aristech-org/tts-client'
import fs from 'fs'

const client = new TtsClient({ host: 'tts.example.com' })
const buffer = await client.audioBuffer({
  text: 'Hello, world!',
  options: { voiceId: 'anne_en_GB' },
})
fs.writeFileSync('path/to/output/file.wav', buffer)
```

There are several examples in the `examples` directory:

- [file.ts](examples/file.ts): Pretty much the same as the example above.
- [streaming.ts](examples/streaming.ts): Demonstrates how to stream audio to a sox process which plays the audio as it is being streamed.
- [voices.ts](examples/models.ts): Demonstrates how to get the available voices from the server.
- [phoneset.ts](examples/phoneset.ts): Demonstrates how to get the phoneset for a voice.
- [transcribe.ts](examples/transcribe.ts): Demonstrates how to get how a voice would pronounce a given word.

You can run the examples directly using `tsx` like this:

1. Create a `.env` file in the [node](.) directory:

```sh
HOST=tts.example.com
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
npx tsx examples/streaming.ts
```

## Build

To rebuild the generated typescript files from the proto file, run:

```bash
npm run generate
```

To build the library, run:

```bash
npm run build
```

