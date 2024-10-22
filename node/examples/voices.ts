import 'dotenv/config'

import { TtsClient, speechAudioFormat_CodecToJSON, speechAudioFormat_ContainerToJSON, voice_GenderToJSON } from '@aristech-org/tts-client'

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new TtsClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})
const voices = await client.listVoices()

for (const voice of voices) {
  // We pull out the enum values and convert them to their string representation for better readability
  const { gender, audio, ...rest } = voice
  console.log({
    ...rest,
    gender: voice_GenderToJSON(gender),
    audio: {
      ...audio,
      container: speechAudioFormat_ContainerToJSON(audio?.container || 0),
      codec: speechAudioFormat_CodecToJSON(audio?.codec || 0),
    },
  })
}