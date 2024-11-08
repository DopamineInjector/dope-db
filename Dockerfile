FROM rust:1.82.0-bookworm AS builder-base
RUN rustup default nightly

FROM debian:bookworm AS release-base

FROM builder-base AS builder
COPY . /app
WORKDIR /app
RUN rustc --version
RUN cargo install --path .

FROM release-base AS release
WORKDIR /
COPY --from=builder /usr/local/cargo/bin/dope-db /usr/local/bin/dope-db
EXPOSE 42069
CMD ["dope-db"]
