# This is *not* sha-locked as ghcr doesn't have arch-agnostic SHAs ATM, AFAICT.
FROM rust:1.92-bookworm AS build

RUN apt-get update && apt-get --assume-yes install protobuf-compiler git clang cmake build-essential llvm-dev libclang-dev jq

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
RUN mkdir -p /var/lib/edera/protect/falco
RUN mv ./target/release/libedera_falco_plugin.so /var/lib/edera/protect/falco/

FROM scratch
COPY --from=build /var/lib/edera/protect/falco/libedera_falco_plugin.so /var/lib/edera/protect/falco/libedera_falco_plugin.so
