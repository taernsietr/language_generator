FROM rust:latest AS builder
WORKDIR /usr/src/language_generator
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release

FROM rust:alpine
RUN addgroup -S angelspeech
RUN adduser -S angelspeech -G angelspeech
USER angelspeech 
WORKDIR /angelspeech
COPY --from=builder /usr/src/app/target/release/language_generator /angelspeech
