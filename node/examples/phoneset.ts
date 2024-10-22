import 'dotenv/config'

import { TtsClient, speechAudioFormat_CodecToJSON, speechAudioFormat_ContainerToJSON, voice_GenderToJSON } from '@aristech-org/tts-client'

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new TtsClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})
const { phoneset } = await client.getPhoneset({ voice: { voiceId: process.env.VOICE_ID || 'anne_en_GB' } })
console.log(JSON.parse(phoneset))