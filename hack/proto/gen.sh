#!/usr/bin/env sh
set -e

# shellcheck source-path=SCRIPTDIR source=versions.sh
. "$(dirname "${0}")/versions.sh"

FAIL=0

if ! command -v buf >/dev/null 2>&1; then
  echo "Generating the protobuf files requires buf. Install it using \"${INSTALL_BUF}\""
  FAIL=1
elif [ "v$(buf --version)" != "${BUF_VERSION}" ]; then
  echo "Generating the protobuf files requires buf ${BUF_VERSION}, you have v$(buf --version) installed. Install it using \"${INSTALL_BUF}\""
  FAIL=1
fi

if ! command -v protoc-gen-prost >/dev/null 2>&1; then
  echo "Generating the protobuf files requires protoc-gen-prost v${PROTOC_GEN_PROST_VERSION}. Install it using \"${INSTALL_PROTOC_GEN_PROST}\""
  FAIL=1
elif [ "$(protoc-gen-prost --version)" != "${PROTOC_GEN_PROST_VERSION}" ]; then
  echo "Generating the protobuf files requires protoc-gen-prost v${PROTOC_GEN_PROST_VERSION}, you have v$(protoc-gen-prost --version) installed. Install it using \"${INSTALL_PROTOC_GEN_PROST}\""
  FAIL=1
fi

if ! command -v protoc-gen-tonic >/dev/null 2>&1; then
  echo "Generating the protobuf files requires protoc-gen-tonic v${PROTOC_GEN_TONIC_VERSION}. Install it using \"${INSTALL_PROTOC_GEN_TONIC}\""
  FAIL=1
elif [ "$(protoc-gen-tonic --version)" != "${PROTOC_GEN_TONIC_VERSION}" ]; then
  echo "Generating the protobuf files requires protoc-gen-tonic v${PROTOC_GEN_TONIC_VERSION}, you have v$(protoc-gen-tonic --version) installed. Install it using \"${INSTALL_PROTOC_GEN_TONIC}\""
  FAIL=1
fi

if ! command -v protoc-gen-prost-serde >/dev/null 2>&1; then
  echo "Generating the protobuf files requires protoc-gen-prost-serde v${PROTOC_GEN_PROST_SERDE_VERSION}. Install it using \"${INSTALL_PROTOC_GEN_PROST_SERDE}\""
  FAIL=1
elif [ "$(protoc-gen-prost-serde --version)" != "${PROTOC_GEN_PROST_SERDE_VERSION}" ]; then
  echo "Generating the protobuf files requires protoc-gen-prost-serde v${PROTOC_GEN_PROST_SERDE_VERSION}, you have v$(protoc-gen-prost-serde --version) installed. Install it using \"${INSTALL_PROTOC_GEN_PROST_SERDE}\""
  FAIL=1
fi

[ "${FAIL}" = "1" ] && exit 1

SCRIPTS_DIR="$(cd -- "$(dirname -- "${0}")" >/dev/null && pwd)"
REPO_ROOT="$(dirname "$(dirname "${SCRIPTS_DIR}")")"

cd "${REPO_ROOT}" >/dev/null || exit

# We need to clean the directories where the protos are generated into
# so that files that should no longer be generated are deleted.
rm -rf src/proto/generated

# One generate command per protobuf package. This allows us to create the protobuf hierarchy
# that the prost generated protobuf files assume, instead of having all the files
# generated into the same folder.

buf generate --template "${SCRIPTS_DIR}/buf.gen.control.yaml"
