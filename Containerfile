FROM rust:1.91-bookworm@sha256:7ccbffbca64e04b1c19647e236b2a41c54eb4ee58891faa43ed70379f264db40 AS build

RUN apt-get update && apt-get --assume-yes install protobuf-compiler git clang cmake build-essential llvm-dev libclang-dev jq

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
RUN mkdir -p /var/lib/edera/protect/falco
RUN mv ./target/release/libedera_falco_plugin.so /var/lib/edera/protect/falco/

FROM scratch
COPY --from=build /var/lib/edera/protect/falco/libedera_falco_plugin.so /var/lib/edera/protect/falco/libedera_falco_plugin.so
