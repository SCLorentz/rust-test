FROM ubuntu:22.04
WORKDIR /app

# install app dependencies
RUN apt-get update && apt-get install -y build-essential
RUN apt-get install -y git

# rust
RUN apt-get install -y curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Settings
# create bashrc
RUN touch ~/.bashrc && chmod u+w ~/.bashrc
# customize bashrc
COPY cmd/prompt /
RUN sh -c 'cat /prompt >> ~/.bashrc'

# final configuration
ENV FLASK_APP=rust_app
EXPOSE 8000
CMD flask run --host 0.0.0.0 --port 8000