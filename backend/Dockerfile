FROM rust:latest AS builder
WORKDIR /usr/src/app
RUN apt-get update
RUN apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/language_generator/target \
    cargo build --target=x86_64-unknown-linux-musl --release 

FROM alpine:latest
RUN addgroup angelspeech
RUN adduser \
    --disabled-password \
    --gecos "" \
    --ingroup angelspeech \
    --no-create-home \
    angelspeech 
USER angelspeech 
WORKDIR /angelspeech
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/language_generator /angelspeech
CMD ["./language_generator"]
