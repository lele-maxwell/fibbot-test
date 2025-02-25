FROM rust:latest 

WORKDIR /action

COPY . .

RUN cargo build --release
RUN carg run 

ENTRYPOINT ["/action/target/release/fibbot-test"]
