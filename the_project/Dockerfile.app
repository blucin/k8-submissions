FROM rust:latest

WORKDIR /app
COPY . .
RUN cargo build --release

ENV APPLICATION_TYPE=app

CMD ["./target/release/the_project"]
