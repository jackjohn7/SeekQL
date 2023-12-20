FROM rust:1.72.0 as BUILDER

RUN apt update && apt upgrade -y

RUN apt install -y nodejs npm

RUN rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/seek_ql

COPY . .

RUN npm install

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM debian:buster-slim

RUN apt update && apt upgrade -y

COPY --from=BUILDER /usr/local/cargo/bin/seek_ql /usr/bin/seek_ql

EXPOSE 5173

CMD ["seek_ql"]
