FROM rust:latest

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .
CMD ["salvo_demo_rbatis"]
