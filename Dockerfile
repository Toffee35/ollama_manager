FROM rust:latest

WORKDIR /usr/src/ollama_manager
COPY . .

RUN cargo install --path .

CMD ["ollama_manager"]
