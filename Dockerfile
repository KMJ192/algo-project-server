FROM rust:1.57

RUN USER=root cargo new --bin algo-project-server
WORKDIR /algo-project-server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/*
RUN cargo build --release

FROM rust:1.57

COPY --from=0 /algo-project-server/target/release/algo-project-server .

EXPOSE 8080

CMD ["./algo-project-server"]
