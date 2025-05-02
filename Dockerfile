# バイナリを alpine で動作させるために rust:1.75.0 でなく rust:1.75.0-alpine でビルドする
FROM --platform=$TARGETPLATFORM rust:1.75.0-alpine3.19 AS electrs_builder
RUN apk add --update --no-cache git clang-dev clang-static cmake gcc g++ linux-headers llvm-dev musl-dev musl-utils
COPY . /app
ENV RUSTFLAGS="-Ctarget-feature=-crt-static"
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      RUST_TOOLCHAIN=1.75.0-aarch64-unknown-linux-musl ;\
    else \
      RUST_TOOLCHAIN=1.75.0-x86_64-unknown-linux-musl ;\
    fi && \
    rustup component add rustfmt --toolchain $RUST_TOOLCHAIN && \
    cd /app && \
    cargo build --release --features liquid --bin electrs

FROM alpine:3.19 AS electrs
RUN apk add --update --no-cache gcc libstdc++
COPY --from=electrs_builder /app/target/release/electrs /bin
CMD ["sh", "-c", "electrs -vvvv --network=liquidregtest"]
