/**
 * Usage: npx tsx examples/streaming.ts [text]
 */
import 'dotenv/config'

import { TtsClient } from '@aristech-org/tts-client'
import { spawn } from 'child_process'

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new TtsClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})
const [stream, voice] = await client.streamAudio({
  text: process.argv[2] || 'Thanks for choosing Aristech. To contact us, dial 0049 6221 438590 or visit aristech.de',
  options: {
    voiceId: process.env.VOICE_ID || 'anne_en_GB'
  }
})

// Open a sox process to play the audio
const sampleRate = voice.audio?.samplerate || 22050
const bitsPerSample = voice.audio?.bitrate || 16
const channels = voice.audio?.channels || 1
const sox = spawn('play', ['-t', 'raw', '-r', sampleRate.toString(), '-b', bitsPerSample.toString(), '-c', channels.toString(), '-e', 'signed-integer', '-'])

stream.on('data', (msg) => {
  sox.stdin.write(Buffer.from(msg.data))
})
stream.on('end', () => {
  sox.stdin.end()
})
stream.on('error', (err) => {
  console.error(err)
})