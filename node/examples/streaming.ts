/**
 * Usage: npx tsx examples/streaming.ts [text]
 */
import 'dotenv/config'

import { SpeechAudioFormat_Codec, SpeechAudioFormat_Container, SpeechResponse, TtsClient } from '@aristech-org/tts-client'

import { spawn } from 'child_process'

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new TtsClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})

const voices = await client.listVoices()
const voice = voices.find((v) => v.voiceId === (process.env.VOICE_ID || 'anne_en_GB'))
if (!voice) {
  throw new Error(`Voice with id ${process.env.VOICE_ID || 'anne_en_GB'} not found`)
}

const stream = await client.streamAudio({
  text: process.argv[2] || 'Thanks for choosing Aristech. For more information about our products visit us at aristech.de',
  options: {
    voiceId: process.env.VOICE_ID || 'anne_en_GB',
    audio: {
      // We don't want any header, just raw PCM data
      container: SpeechAudioFormat_Container.RAW,
      codec: SpeechAudioFormat_Codec.PCM,
    }
  }
})

// Open a sox process to play the audio
const sampleRate = voice.audio?.samplerate || 22050
const bitsPerSample = voice.audio?.bitDepth || 16
const channels = voice.audio?.channels || 1
const sox = spawn('play', ['-q', '-t', 'raw', '-r', String(sampleRate), '-e', 'signed', '-b', String(bitsPerSample), '-c', String(channels), '-L', '-'])

sox.on('exit', (code, signal) => {
  console.log(`Sox process exited with code ${code} and signal ${signal}`)
})

stream.on('data', (msg: SpeechResponse) => {
  const chunk = Buffer.from(msg.data)
  sox.stdin.write(chunk)
})

stream.on('error', (err) => {
  console.error(err)
})

stream.on('close', () => {
  sox.stdin.end()
})
