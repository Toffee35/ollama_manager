FROM rust:latest

WORKDIR /usr/src/ollama_bot
COPY . .

RUN cargo install --path .

CMD ["ollama_bot"]
