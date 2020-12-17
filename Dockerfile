# バイナリを alpine で動作させるために rust:1.48.0 でなく rust:1.48.0-alpine でビルドする
FROM rust:1.48.0-alpine as electrs_builder
COPY . /app
ENV RUSTFLAGS="-Ctarget-feature=-crt-static"
RUN apk add git clang cmake && \
    # rust:1.48.0-alpine でビルドするのに必要なパッケージ
    apk add gcc g++ linux-headers llvm-dev musl-dev musl-utils && \
    rustup component add rustfmt --toolchain 1.48.0-x86_64-unknown-linux-musl && \
    cd /app && \
    cargo build --release --features liquid --bin electrs
FROM alpine as electrs
RUN apk add gcc libstdc++
COPY --from=electrs_builder /app/target/release/electrs /bin
# CMD sh -c "electrs -vv --timestamp --jsonrpc-import --network=liquidregtest --cookie=elements:elements --daemon-rpc-addr=172.32.0.3:7040 --http-addr=0.0.0.0:3002"
