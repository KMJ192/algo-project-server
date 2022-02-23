FROM rust:1.58

RUN USER=root cargo new --bin algo-project-server
WORKDIR /algo-project-server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/*
RUN cargo build --release

FROM rust:1.58

COPY --from=0 /algo-project-server/target/release/algo-project-server .

CMD ["algo-project-server"]
