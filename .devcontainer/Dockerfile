FROM debian:bookworm

COPY cmd/.bash /root/.bash
RUN rm -rf ~/.bashrc && mv ~/.bash ~/.bashrc

RUN apt-get update && apt-get install -y build-essential git curl wget \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN BIN="/usr/local/bin" \
    && UPX="${BIN}/upx-4.2.4-arm64_linux.tar.xz" \
    && wget -O ${UPX} --no-check-certificate https://github.com/upx/upx/releases/download/v4.2.4/upx-4.2.4-arm64_linux.tar.xz \
    && tar xf ${UPX} -C ${BIN}

ENV FLASK_APP=rust_app
EXPOSE 8000
ENTRYPOINT flask run --host 0.0.0.0 --port 8000