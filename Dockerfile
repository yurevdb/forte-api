FROM rust:latest as build

RUN USER=root cargo new --bin forte
WORKDIR /forte

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/forte*
RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=build /forte/target/release/forte .

EXPOSE 8080

CMD ["./forte"]
