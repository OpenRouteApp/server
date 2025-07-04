FROM clux/muslrust:1.87.0-stable AS builder

WORKDIR /app

COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/babieca /usr/local/bin/babieca

ENTRYPOINT ["/usr/local/bin/vote-manager"]
