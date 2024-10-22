/**
 * Usage: npx tsx examples/file.ts [text]
 */
import 'dotenv/config'

import { TtsClient } from '@aristech-org/tts-client'
import fs from 'fs'

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new TtsClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})
const buffer = await client.audioBuffer({
  text: process.argv[2] || 'Thanks for choosing Aristech. To contact us, dial 0049 6221 438590 or visit aristech.de',
  options: {
    voiceId: process.env.VOICE_ID || 'anne_en_GB'
  }
})

fs.writeFileSync('output.wav', buffer)