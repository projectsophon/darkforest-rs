FROM rust
WORKDIR /darkforest-rs

COPY . .
RUN cargo build --release --bin mimc-fast

EXPOSE 8000
CMD ["./target/release/mimc-fast"]
