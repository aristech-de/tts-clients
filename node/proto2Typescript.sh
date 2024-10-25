#!/usr/bin/env bash
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
WORKSPACE_DIR=$(cd -- "${SCRIPT_DIR}/.." &>/dev/null && pwd)

# This script converts the protocol buffer files to typescript
OUT_DIR="$SCRIPT_DIR/src/generated"
# Directory containing the proto files
PROTO_DIR="$WORKSPACE_DIR/protos"

rm -rf ${OUT_DIR}
mkdir -p ${OUT_DIR}

# Generate typescript from proto files using ts-proto
protoc \
  --plugin=./node_modules/.bin/protoc-gen-ts_proto \
  --ts_proto_out=${OUT_DIR} \
  --ts_proto_opt=esModuleInterop \
  --ts_proto_opt=importSuffix=.js \
  --ts_proto_opt=outputServices=grpc-js \
  --proto_path=${PROTO_DIR} \
  ${PROTO_DIR}/*.proto
