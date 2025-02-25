FROM rust:1.75

WORKDIR /action

COPY . .

#RUN cargo build --release
RUN cargo run 

#ENTRYPOINT ["/action/target/release/fibbot-test"]
