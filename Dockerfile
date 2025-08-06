ARG RUST_VERSION=1.88
ARG APP_NAME=keter-rest

FROM rust:${RUST_VERSION}-slim-bookworm AS builder
ARG RUST_VERSION
ARG APP_NAME

WORKDIR /app

COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN apt update && \
    apt install -y pkg-config libssl-dev

RUN \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/${APP_NAME} /

FROM debian:bookworm-slim AS final

ARG RUST_VERSION
ARG APP_NAME

RUN apt update && \
    apt install -y pkg-config libssl-dev

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser

COPY --from=builder /${APP_NAME} /usr/local/bin
RUN chown appuser /usr/local/bin/${APP_NAME}

WORKDIR /opt/${APP_NAME}
RUN chown -R appuser /opt/${APP_NAME}

USER appuser
ENV RUN_CMD=${APP_NAME}

EXPOSE 3000
ENTRYPOINT $RUN_CMD
