# Based on https://github.com/DazWilkin/do-apps-rust
FROM rustlang/rust:nightly-slim as builder

RUN USER=root cargo new --bin mimc-fast

WORKDIR /mimc-fast

COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/mimc_fast*

RUN cargo build --release


FROM debian:buster-slim as runtime

WORKDIR /bin

# Copy from builder and rename to 'server'
COPY --from=builder /mimc-fast/target/release/mimc-fast ./server

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    USER=appuser

RUN groupadd ${USER} \
    && useradd -g ${USER} ${USER} && \
    chown -R ${USER}:${USER} /bin

USER ${USER}

ENTRYPOINT ["./server"]
