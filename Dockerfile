FROM rust:slim-bullseye as build
LABEL authors="nat"
RUN USER=root apt-get update -y && apt-get -y install pkg-config libssl-dev
RUN USER=root cargo new --bin mempaste-api
WORKDIR /mempaste-api

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
RUN cargo build --release

RUN rm ./target/release/deps/mempaste_api*

FROM rust:slim-bullseye

WORKDIR /app
COPY --from=build /mempaste-api/target/release/mempaste-api ./mempaste-api
RUN touch ./.env
CMD ["./mempaste-api"]
