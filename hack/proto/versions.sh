#!/usr/bin/env sh
# shellcheck disable=SC2034
set -e

PROTOC_GEN_PROST_REPOSITORY="https://github.com/neoeinstein/protoc-gen-prost.git"
PROTOC_GEN_PROST_COMMIT=6915e08b082a15feb260062fe82d049edc0eadd6
BUF_COMMIT=69a3227530199878cc50df6ce889b176b498e077

BUF_VERSION=v1.56.0
PROTOC_GEN_PROST_VERSION=0.5.0
PROTOC_GEN_TONIC_VERSION=0.5.0
PROTOC_GEN_PROST_SERDE_VERSION=0.4.0

INSTALL_BUF="go install github.com/bufbuild/buf/cmd/buf@${BUF_COMMIT}"
INSTALL_PROTOC_GEN_PROST="cargo install --locked --git ${PROTOC_GEN_PROST_REPOSITORY} --rev ${PROTOC_GEN_PROST_COMMIT} protoc-gen-prost"
INSTALL_PROTOC_GEN_TONIC="cargo install --locked --git ${PROTOC_GEN_PROST_REPOSITORY} --rev ${PROTOC_GEN_PROST_COMMIT} protoc-gen-tonic"
INSTALL_PROTOC_GEN_PROST_SERDE="cargo install --locked --git ${PROTOC_GEN_PROST_REPOSITORY} --rev ${PROTOC_GEN_PROST_COMMIT} protoc-gen-prost-serde"
