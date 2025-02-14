FROM debian:bookworm
WORKDIR /app

RUN apt-get update && apt-get install -y build-essential git curl \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

COPY cmd/bash /root/bash
RUN rm -rf ~/.bashrc && mv ~/bash ~/.bashrc

ENV FLASK_APP=rust_app
EXPOSE 8000
ENTRYPOINT flask run --host 0.0.0.0 --port 8000