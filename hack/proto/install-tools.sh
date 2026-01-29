#!/usr/bin/env sh
# shellcheck disable=SC2086
set -e

# shellcheck source-path=SCRIPTDIR source=versions.sh
. "$(dirname "${0}")/versions.sh"

[ -z "${ADD_GO_FLAGS}" ] && ADD_GO_FLAGS=""
[ -z "${ADD_CARGO_FLAGS}" ] && ADD_CARGO_FLAGS="--force"

${INSTALL_BUF} ${ADD_GO_FLAGS}
${INSTALL_PROTOC_GEN_GO} ${ADD_GO_FLAGS}
${INSTALL_PROTOC_GEN_GO_GRPC} ${ADD_GO_FLAGS}
${INSTALL_PROTOC_GEN_PROST} ${ADD_CARGO_FLAGS}
${INSTALL_PROTOC_GEN_TONIC} ${ADD_CARGO_FLAGS}
${INSTALL_PROTOC_GEN_PROST_SERDE} ${ADD_CARGO_FLAGS}
