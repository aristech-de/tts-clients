# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc
import warnings

from . import TTSServices_pb2 as TTSServices__pb2
from . import TTSTypes_pb2 as TTSTypes__pb2

GRPC_GENERATED_VERSION = '1.67.1'
GRPC_VERSION = grpc.__version__
_version_not_supported = False

try:
    from grpc._utilities import first_version_is_lower
    _version_not_supported = first_version_is_lower(GRPC_VERSION, GRPC_GENERATED_VERSION)
except ImportError:
    _version_not_supported = True

if _version_not_supported:
    raise RuntimeError(
        f'The grpc package installed is at version {GRPC_VERSION},'
        + f' but the generated code in TTSServices_pb2_grpc.py depends on'
        + f' grpcio>={GRPC_GENERATED_VERSION}.'
        + f' Please upgrade your grpc module to grpcio>={GRPC_GENERATED_VERSION}'
        + f' or downgrade your generated code using grpcio-tools<={GRPC_VERSION}.'
    )


class SpeechServiceStub(object):
    """Service that implements Aristech Speech-API (TTS-API, ariTTS)
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetSpeech = channel.unary_stream(
                '/aristech.tts.SpeechService/GetSpeech',
                request_serializer=TTSServices__pb2.SpeechRequest.SerializeToString,
                response_deserializer=TTSServices__pb2.SpeechResponse.FromString,
                _registered_method=True)
        self.ControlServer = channel.stream_stream(
                '/aristech.tts.SpeechService/ControlServer',
                request_serializer=TTSServices__pb2.ServerCommand.SerializeToString,
                response_deserializer=TTSServices__pb2.ServerCommandResponse.FromString,
                _registered_method=True)
        self.GetVoiceList = channel.unary_stream(
                '/aristech.tts.SpeechService/GetVoiceList',
                request_serializer=TTSServices__pb2.VoiceListRequest.SerializeToString,
                response_deserializer=TTSTypes__pb2.Voice.FromString,
                _registered_method=True)
        self.GetPhoneset = channel.unary_unary(
                '/aristech.tts.SpeechService/GetPhoneset',
                request_serializer=TTSServices__pb2.PhonesetRequest.SerializeToString,
                response_deserializer=TTSServices__pb2.PhonesetResponse.FromString,
                _registered_method=True)
        self.GetTranscription = channel.unary_unary(
                '/aristech.tts.SpeechService/GetTranscription',
                request_serializer=TTSServices__pb2.TranscriptionRequest.SerializeToString,
                response_deserializer=TTSServices__pb2.TranscriptionResponse.FromString,
                _registered_method=True)


class SpeechServiceServicer(object):
    """Service that implements Aristech Speech-API (TTS-API, ariTTS)
    """

    def GetSpeech(self, request, context):
        """Performs Text-to-Speech with the given SpeechRequest and streams back the
        audio as packets of type SpeechResponse.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ControlServer(self, request_iterator, context):
        """Performs Text-to-Speech and streams back the audio. Adds the capability to
        stop the speech synthesis and free a port during synthesis.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetVoiceList(self, request, context):
        """Returns available voices as stream.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetPhoneset(self, request, context):
        """Returns the phoneset for a given voice
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetTranscription(self, request, context):
        """Returns the transcription for a word for a given voice
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_SpeechServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'GetSpeech': grpc.unary_stream_rpc_method_handler(
                    servicer.GetSpeech,
                    request_deserializer=TTSServices__pb2.SpeechRequest.FromString,
                    response_serializer=TTSServices__pb2.SpeechResponse.SerializeToString,
            ),
            'ControlServer': grpc.stream_stream_rpc_method_handler(
                    servicer.ControlServer,
                    request_deserializer=TTSServices__pb2.ServerCommand.FromString,
                    response_serializer=TTSServices__pb2.ServerCommandResponse.SerializeToString,
            ),
            'GetVoiceList': grpc.unary_stream_rpc_method_handler(
                    servicer.GetVoiceList,
                    request_deserializer=TTSServices__pb2.VoiceListRequest.FromString,
                    response_serializer=TTSTypes__pb2.Voice.SerializeToString,
            ),
            'GetPhoneset': grpc.unary_unary_rpc_method_handler(
                    servicer.GetPhoneset,
                    request_deserializer=TTSServices__pb2.PhonesetRequest.FromString,
                    response_serializer=TTSServices__pb2.PhonesetResponse.SerializeToString,
            ),
            'GetTranscription': grpc.unary_unary_rpc_method_handler(
                    servicer.GetTranscription,
                    request_deserializer=TTSServices__pb2.TranscriptionRequest.FromString,
                    response_serializer=TTSServices__pb2.TranscriptionResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'aristech.tts.SpeechService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))
    server.add_registered_method_handlers('aristech.tts.SpeechService', rpc_method_handlers)


 # This class is part of an EXPERIMENTAL API.
class SpeechService(object):
    """Service that implements Aristech Speech-API (TTS-API, ariTTS)
    """

    @staticmethod
    def GetSpeech(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_stream(
            request,
            target,
            '/aristech.tts.SpeechService/GetSpeech',
            TTSServices__pb2.SpeechRequest.SerializeToString,
            TTSServices__pb2.SpeechResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def ControlServer(request_iterator,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.stream_stream(
            request_iterator,
            target,
            '/aristech.tts.SpeechService/ControlServer',
            TTSServices__pb2.ServerCommand.SerializeToString,
            TTSServices__pb2.ServerCommandResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def GetVoiceList(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_stream(
            request,
            target,
            '/aristech.tts.SpeechService/GetVoiceList',
            TTSServices__pb2.VoiceListRequest.SerializeToString,
            TTSTypes__pb2.Voice.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def GetPhoneset(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/aristech.tts.SpeechService/GetPhoneset',
            TTSServices__pb2.PhonesetRequest.SerializeToString,
            TTSServices__pb2.PhonesetResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)

    @staticmethod
    def GetTranscription(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/aristech.tts.SpeechService/GetTranscription',
            TTSServices__pb2.TranscriptionRequest.SerializeToString,
            TTSServices__pb2.TranscriptionResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)


class DebugServiceStub(object):
    """Internal Use Only: debug access to engine
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.ProcessData = channel.unary_stream(
                '/aristech.tts.DebugService/ProcessData',
                request_serializer=TTSServices__pb2.SpeechRequest.SerializeToString,
                response_deserializer=TTSServices__pb2.SpeechResponse.FromString,
                _registered_method=True)


class DebugServiceServicer(object):
    """Internal Use Only: debug access to engine
    """

    def ProcessData(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_DebugServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'ProcessData': grpc.unary_stream_rpc_method_handler(
                    servicer.ProcessData,
                    request_deserializer=TTSServices__pb2.SpeechRequest.FromString,
                    response_serializer=TTSServices__pb2.SpeechResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'aristech.tts.DebugService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))
    server.add_registered_method_handlers('aristech.tts.DebugService', rpc_method_handlers)


 # This class is part of an EXPERIMENTAL API.
class DebugService(object):
    """Internal Use Only: debug access to engine
    """

    @staticmethod
    def ProcessData(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_stream(
            request,
            target,
            '/aristech.tts.DebugService/ProcessData',
            TTSServices__pb2.SpeechRequest.SerializeToString,
            TTSServices__pb2.SpeechResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)
