FROM rust:latest

WORKDIR /usr/src/SAPI_BACKEND

COPY . .

RUN cargo build --release

EXPOSE 3000
CMD cargo run