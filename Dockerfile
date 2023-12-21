FROM rust:1.72.0 as BUILDER

RUN apt update && apt upgrade -y

RUN apt install -y nodejs npm

RUN rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/seek_ql

COPY . .

# install npm dependencies
RUN npm install

# needed for avoiding "missing glibc" nonsense
RUN rustup target add x86_64-unknown-linux-musl

# specifying target to ensure compatibility for the container (debian)
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM debian:buster-slim
# Quite literally just move the compiled executable into bin directory
#  and execute it. Resulting container is tiny.

RUN apt update && apt upgrade -y

COPY --from=BUILDER /usr/local/cargo/bin/seek_ql /usr/bin/seek_ql
COPY --from=BUILDER /usr/src/seek_ql/public public

EXPOSE 5173

CMD ["seek_ql"]
