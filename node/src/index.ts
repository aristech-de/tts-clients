import * as grpc from '@grpc/grpc-js'

import { DeepPartial, SpeechAudioFormat_Codec, SpeechAudioFormat_Container, Voice } from './generated/TTSTypes.js'
import { PhonesetRequest, PhonesetResponse, SpeechRequest, SpeechResponse, SpeechServiceClient, TranscriptionRequest, TranscriptionResponse, VoiceListRequest } from './generated/TTSServices.js'

import fs from 'fs'

export * from './generated/TTSTypes.js'

export type Stream = grpc.ClientReadableStream<SpeechResponse>

export interface ConnectionOptions {
  /**
   * The Aristech TTS-Server uri e.g. tts.example.com
   */
  host?: string
  /**
   * Whether to use SSL/TLS. Automatically enabled when rootCert is provided
   */
  ssl?: boolean
  /**
   * Allows providing a custom root certificate that might not exist
   * in the root certificate chain
   */
  rootCert?: string
  /**
   * Optionally instead of providing a root cert path via `rootCert` the root cert content can be provided directly
   */
  rootCertContent?: string
  /**
   * Further grpc client options
   */
  grpcClientOptions?: grpc.ClientOptions
  /**
   * Authentication options.
   * **Note:** Can only be used in combination with SSL/TLS.
   */
  auth?: {
    token: string
    secret: string
  }
}

export class TtsClient {
  private cOptions: ConnectionOptions

  constructor(options: ConnectionOptions) {
    this.cOptions = options
  }

  /**
   * Lists all available voices.
   */
  listVoices(request?: DeepPartial<VoiceListRequest>): Promise<Array<Voice>> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = VoiceListRequest.create(request)
      const stream = client.getVoiceList(req)
      const voices: Voice[] = []
      stream.on('data', (voice: Voice) => {
        voices.push(voice)
      })
      stream.on('end', () => {
        res(voices)
      })
      stream.on('error', (err) => {
        rej(err)
      })
    })
  }

  /**
   * Creates a stream of audio data from the given text.
   * @param request The request object
   * @returns A tuple containing the stream and the voice used for the audio generation
   */
  streamAudio(request: DeepPartial<SpeechRequest>): Promise<[Stream, Voice]> {
    return new Promise<[Stream, Voice]>(async (res, rej) => {
      const client = this.getClient()
      const req = SpeechRequest.create({
        ...request,
        inputType: 'SSML',
        outputType: 'AUDIO',
        options: {
          ...request.options,
          audio: {
            ...request.options?.audio,
            codec: SpeechAudioFormat_Codec.PCM,
            container: SpeechAudioFormat_Container.WAV,
          }
        }
      })
      // Get the voiceId from the request
      const voiceId = req.options?.voiceId
      if (!voiceId) {
        rej(new Error('voiceId is required'))
        return
      }
      // Get the voice audio specs
      const voices = await this.listVoices()
      const voice = voices.find((v) => v.voiceId === voiceId)
      if (!voice) {
        rej(new Error(`Voice with id "${voiceId}" not found`))
        return
      }
      const stream = client.getSpeech(req)
      res([stream, voice])
    })
  }

  /**
   * Creates an audio buffer from the given text.
   * @param request The request object
   * @returns The audio buffer
   */
  synthesize(request: DeepPartial<SpeechRequest>): Promise<Buffer> {
    return new Promise<Buffer>(async (res, rej) => {
      const [stream, voice] = await this.streamAudio(request)
      const rawChunks: Buffer[] = []
      stream.on('data', (msg: SpeechResponse) => {
        const chunk = Buffer.from(msg.data)
        rawChunks.push(chunk)
      })
      stream.on('end', () => {
        const audioBuffer = Buffer.concat(rawChunks)
        const requestedFormat = request?.options?.audio?.container || SpeechAudioFormat_Container.WAV
        // Wave headers need to be prepended to the audio data
        // because while generating the audio data, the data length is not known
        if (requestedFormat === SpeechAudioFormat_Container.WAV) {
          const sampleRate = voice.audio.samplerate
          const bitsPerSample = voice.audio.bitrate
          const header = createWaveHeader(audioBuffer.length, sampleRate, 1, bitsPerSample)
          res(Buffer.concat([header, audioBuffer]))
          return
        }
        res(audioBuffer)
      })
      stream.on('error', (err) => {
        rej(err)
      })
    })
  }

  /**
   * This is an alias for the `synthesize` method.
   */
  audioBuffer(request: DeepPartial<SpeechRequest>): Promise<Buffer> {
    return this.synthesize(request)
  }

  /**
   * Retrieves the phoneset for the given voice.
   * @param request The request object
   * @returns The phoneset response
   */
  getPhoneset(request: DeepPartial<PhonesetRequest>): Promise<PhonesetResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = PhonesetRequest.create(request)
      client.getPhoneset(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
   * Retrieves the transcription for the given request.
   * @param request The request object
   * @returns The transcription response
   */
  getTranscription(request: DeepPartial<TranscriptionRequest>): Promise<TranscriptionResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = TranscriptionRequest.create(request)
      client.getTranscription(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  private getClient() {
    const { rootCert: rootCertPath, rootCertContent, auth, grpcClientOptions } = this.cOptions
    let host = this.cOptions.host || 'localhost:8423'
    let ssl = this.cOptions.ssl === true
    let rootCert: Buffer | null = null
    if (rootCertContent) {
      rootCert = Buffer.from(rootCertContent)
    } else if (rootCertPath) {
      rootCert = fs.readFileSync(rootCertPath)
    }
    const sslExplicit = typeof this.cOptions.ssl === 'boolean' || !!rootCert
    const portRe = /[^:]+:([0-9]+)$/
    if (portRe.test(host)) {
      // In case a port was provided but ssl was not specified
      // ssl is assumed when the port matches 9424
      const [, portStr] = host.match(portRe)!
      const hostPort = parseInt(portStr, 10)
      if (!sslExplicit) {
        if (hostPort === 9424) {
          ssl = true
        } else {
          ssl = false
        }
      }
    } else {
      // In case no port was provided, depending on the ssl settings
      // at the default non ssl port 8423 or ssl port 8424
      if (sslExplicit && ssl) {
        host = `${host}:8424`
      } else {
        host = `${host}:8423`
      }
    }

    let creds = grpc.credentials.createInsecure()
    if (ssl || rootCert) {
      creds = grpc.credentials.createSsl(rootCert)
      if (auth) {
        const callCreds = grpc.credentials.createFromMetadataGenerator(
          (_, cb) => {
            const meta = new grpc.Metadata()
            meta.add('token', auth.token)
            meta.add('secret', auth.secret)
            cb(null, meta)
          },
        )
        creds = grpc.credentials.combineChannelCredentials(creds, callCreds)
      }
    }
    return new SpeechServiceClient(host, creds, grpcClientOptions)
  }
}

/**
 * A helper function to create a WAV header for the given audio data.
 * @param dataLength The length of the audio data in bytes
 * @param sampleRate Sample rate in Hz
 * @param numChannels Number of channels
 * @param bitsPerSample Bits per sample
 * @returns The WAV header as a buffer
 */
export function createWaveHeader(
  dataLength: number,
  sampleRate: number,
  numChannels: number,
  bitsPerSample: number,
): Buffer {
  const byteRate = (sampleRate * numChannels * bitsPerSample) / 8
  const blockAlign = (numChannels * bitsPerSample) / 8
  const headerSize = 44; // Standard WAV header size

  const buffer = new ArrayBuffer(headerSize)
  const view = new DataView(buffer)

  // RIFF chunk descriptor
  writeString(view, 0, "RIFF") // ChunkID
  view.setUint32(4, 36 + dataLength, true) // ChunkSize = 36 + dataLength
  writeString(view, 8, "WAVE") // Format

  // "fmt " sub-chunk
  writeString(view, 12, "fmt ") // Subchunk1ID
  view.setUint32(16, 16, true) // Subchunk1Size (PCM = 16)
  view.setUint16(20, 1, true) // AudioFormat (PCM = 1)
  view.setUint16(22, numChannels, true) // NumChannels (Mono = 1)
  view.setUint32(24, sampleRate, true) // SampleRate
  view.setUint32(28, byteRate, true) // ByteRate
  view.setUint16(32, blockAlign, true) // BlockAlign
  view.setUint16(34, bitsPerSample, true) // BitsPerSample

  // "data" sub-chunk
  writeString(view, 36, "data") // Subchunk2ID
  view.setUint32(40, dataLength, true) // Subchunk2Size = dataLength

  return Buffer.from(buffer)
}

function writeString(view: DataView, offset: number, str: string) {
  for (let i = 0; i < str.length; i++) {
    view.setUint8(offset + i, str.charCodeAt(i))
  }
}