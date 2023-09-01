FROM rust:1.72.0-slim-buster AS builder

RUN apt-get update  \
    && apt-get install -y pkg-config libssl-dev
RUN USER=root cargo new --bin rust-crud-api
WORKDIR ./rust-crud-api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
RUN rm ./target/release/deps/rust_crud_api*
RUN cargo build --release

FROM debian:buster-slim AS runtime
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rust-crud-api/target/release/rust-crud-api ${APP}/rust-crud-api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./rust-crud-api"]