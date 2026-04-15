import * as grpc from '@grpc/grpc-js'

import { ClearCacheRequest, ClearCacheResponse, PhonesetRequest, PhonesetResponse, SpeechRequest, SpeechResponse, SpeechServiceClient, SsmlDocumentationRequest, SsmlDocumentationResponse, TranscriptionRequest, TranscriptionResponse, VoiceListRequest } from './generated/TTSServices.js'
import { DeepPartial, Voice } from './generated/TTSTypes.js'

import fs from 'fs'

export * from './generated/TTSTypes.js'
export { PhonesetRequest, PhonesetResponse, SpeechRequest, SpeechResponse, SpeechServiceClient, SsmlDocumentationRequest, SsmlDocumentationResponse, TranscriptionRequest, TranscriptionResponse, VoiceListRequest } from './generated/TTSServices.js'

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
   * @returns The response stream
   */
  streamAudio(request: DeepPartial<SpeechRequest>): Promise<Stream> {
    return new Promise<Stream>(async (res, rej) => {
      const client = this.getClient()
      const req = SpeechRequest.create(request)
      const stream = client.getSpeech(req)
      res(stream)
    })
  }

  /**
   * Creates an audio buffer from the given text.
   * @param request The request object
   * @returns The audio buffer
   */
  synthesize(request: DeepPartial<SpeechRequest>): Promise<Buffer> {
    return new Promise<Buffer>(async (res, rej) => {
      const stream = await this.streamAudio(request)
      const rawChunks: Buffer[] = []
      stream.on('data', (msg: SpeechResponse) => {
        const chunk = Buffer.from(msg.data)
        rawChunks.push(chunk)
      })
      stream.on('end', () => {
        res(Buffer.concat(rawChunks))
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

  /**
   * Clears the cache of the server, removing all cached audio data.
   * @param request The request object
   * @returns The clear cache response
   */
  clearCache(request: DeepPartial<ClearCacheRequest> = {}): Promise<ClearCacheResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = ClearCacheRequest.create(request)
      client.clearCache(req, (err, response) => {
        if (err) {
          rej(err)
          return
        }
        res(response)
      })
    })
  }

  /**
   * Retrieves SSML documentation for a specific voice.
   * @param request The request object containing voice_id and optional locale
   * @returns The SSML documentation response
   */
  getSsmlDocumentation(request: DeepPartial<SsmlDocumentationRequest>): Promise<SsmlDocumentationResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = SsmlDocumentationRequest.create(request)
      client.getSsmlDocumentation(req, (err, response) => {
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
      // ssl is assumed when the port matches 8424
      const [, portStr] = host.match(portRe)!
      const hostPort = parseInt(portStr, 10)
      if (!sslExplicit) {
        if (hostPort === 8424) {
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