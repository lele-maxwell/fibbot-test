FROM rust:latest 

WORKDIR /action

COPY . .

#RUN cargo build --release
RUN cargo run 

#ENTRYPOINT ["/action/target/release/fibbot-test"]
