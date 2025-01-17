# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: TTSServices.proto
# Protobuf Python Version: 5.27.2
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    27,
    2,
    '',
    'TTSServices.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from . import TTSTypes_pb2 as TTSTypes__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x11TTSServices.proto\x12\x0c\x61ristech.tts\x1a\x0eTTSTypes.proto\"\x8e\x01\n\rSpeechRequest\x12\x0c\n\x04text\x18\x01 \x01(\t\x12\x32\n\x07options\x18\x02 \x01(\x0b\x32!.aristech.tts.SpeechRequestOption\x12\x12\n\nparameters\x18\x03 \x01(\t\x12\x12\n\ninput_type\x18\x04 \x01(\t\x12\x13\n\x0boutput_type\x18\x05 \x01(\t\"5\n\x0fPhonesetRequest\x12\"\n\x05voice\x18\x01 \x01(\x0b\x32\x13.aristech.tts.Voice\"E\n\x10PhonesetResponse\x12\x0e\n\x06status\x18\x01 \x01(\x03\x12\x0f\n\x07message\x18\x02 \x01(\t\x12\x10\n\x08phoneset\x18\x03 \x01(\t\"H\n\x14TranscriptionRequest\x12\"\n\x05voice\x18\x01 \x01(\x0b\x32\x13.aristech.tts.Voice\x12\x0c\n\x04word\x18\x02 \x01(\t\"O\n\x15TranscriptionResponse\x12\x0e\n\x06status\x18\x01 \x01(\x03\x12\x0f\n\x07message\x18\x02 \x01(\t\x12\x15\n\rtranscription\x18\x03 \x01(\t\"\x8b\x01\n\rServerCommand\x12/\n\x0c\x63ommand_type\x18\x01 \x01(\x0e\x32\x19.aristech.tts.CommandType\x12\x14\n\x0c\x63ommand_data\x18\x02 \x01(\x0c\x12\x33\n\x0espeech_request\x18\x03 \x01(\x0b\x32\x1b.aristech.tts.SpeechRequest\"\xc0\x01\n\x15ServerCommandResponse\x12\x38\n\rresponse_type\x18\x01 \x01(\x0e\x32!.aristech.tts.CommandResponseType\x12\x0e\n\x06status\x18\x02 \x01(\x03\x12\x0f\n\x07message\x18\x03 \x01(\t\x12\x15\n\rresponse_data\x18\x04 \x01(\x0c\x12\x35\n\x0fspeech_response\x18\x05 \x03(\x0b\x32\x1c.aristech.tts.SpeechResponse\"W\n\x0eSpeechResponse\x12\x0e\n\x06status\x18\x01 \x01(\x03\x12\x0c\n\x04\x64\x61ta\x18\x02 \x01(\x0c\x12\x12\n\ninput_type\x18\x03 \x01(\t\x12\x13\n\x0boutput_type\x18\x04 \x01(\t\">\n\x10VoiceListRequest\x12*\n\x06locale\x18\x01 \x01(\x0b\x32\x1a.aristech.tts.SpeechLocale*F\n\x0b\x43ommandType\x12\x11\n\rSTART_REQUEST\x10\x00\x12\x10\n\x0cSTOP_REQUEST\x10\x01\x12\x12\n\x0eSTATUS_REQUEST\x10\x02*e\n\x13\x43ommandResponseType\x12\x11\n\rSTARTRESPONSE\x10\x00\x12\x10\n\x0cSTOPRESPONSE\x10\x01\x12\x12\n\x0eSTATUSRESPONSE\x10\x02\x12\x15\n\x11SYNTHESISRESPONSE\x10\x03\x32\xac\x03\n\rSpeechService\x12J\n\tGetSpeech\x12\x1b.aristech.tts.SpeechRequest\x1a\x1c.aristech.tts.SpeechResponse\"\x00\x30\x01\x12W\n\rControlServer\x12\x1b.aristech.tts.ServerCommand\x1a#.aristech.tts.ServerCommandResponse\"\x00(\x01\x30\x01\x12G\n\x0cGetVoiceList\x12\x1e.aristech.tts.VoiceListRequest\x1a\x13.aristech.tts.Voice\"\x00\x30\x01\x12N\n\x0bGetPhoneset\x12\x1d.aristech.tts.PhonesetRequest\x1a\x1e.aristech.tts.PhonesetResponse\"\x00\x12]\n\x10GetTranscription\x12\".aristech.tts.TranscriptionRequest\x1a#.aristech.tts.TranscriptionResponse\"\x00\x32\\\n\x0c\x44\x65\x62ugService\x12L\n\x0bProcessData\x12\x1b.aristech.tts.SpeechRequest\x1a\x1c.aristech.tts.SpeechResponse\"\x00\x30\x01\x42\x37\n\x19\x64\x65.aristech.tts.serializeB\x12SpeechServiceProtoP\x01\xa2\x02\x03SSPb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'TTSServices_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\031de.aristech.tts.serializeB\022SpeechServiceProtoP\001\242\002\003SSP'
  _globals['_COMMANDTYPE']._serialized_start=967
  _globals['_COMMANDTYPE']._serialized_end=1037
  _globals['_COMMANDRESPONSETYPE']._serialized_start=1039
  _globals['_COMMANDRESPONSETYPE']._serialized_end=1140
  _globals['_SPEECHREQUEST']._serialized_start=52
  _globals['_SPEECHREQUEST']._serialized_end=194
  _globals['_PHONESETREQUEST']._serialized_start=196
  _globals['_PHONESETREQUEST']._serialized_end=249
  _globals['_PHONESETRESPONSE']._serialized_start=251
  _globals['_PHONESETRESPONSE']._serialized_end=320
  _globals['_TRANSCRIPTIONREQUEST']._serialized_start=322
  _globals['_TRANSCRIPTIONREQUEST']._serialized_end=394
  _globals['_TRANSCRIPTIONRESPONSE']._serialized_start=396
  _globals['_TRANSCRIPTIONRESPONSE']._serialized_end=475
  _globals['_SERVERCOMMAND']._serialized_start=478
  _globals['_SERVERCOMMAND']._serialized_end=617
  _globals['_SERVERCOMMANDRESPONSE']._serialized_start=620
  _globals['_SERVERCOMMANDRESPONSE']._serialized_end=812
  _globals['_SPEECHRESPONSE']._serialized_start=814
  _globals['_SPEECHRESPONSE']._serialized_end=901
  _globals['_VOICELISTREQUEST']._serialized_start=903
  _globals['_VOICELISTREQUEST']._serialized_end=965
  _globals['_SPEECHSERVICE']._serialized_start=1143
  _globals['_SPEECHSERVICE']._serialized_end=1571
  _globals['_DEBUGSERVICE']._serialized_start=1573
  _globals['_DEBUGSERVICE']._serialized_end=1665
# @@protoc_insertion_point(module_scope)
