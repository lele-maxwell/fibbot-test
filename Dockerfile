
FROM rust:latest


WORKDIR /app


COPY . /app


RUN cargo build --release

CMD ["./target/release/fibbot-test"]
