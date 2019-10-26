# define ubuntu version, you can use --build-arg
ARG ubuntu_version="19.10"
FROM ubuntu:${ubuntu_version}

# build name space
ENV NAMESPACE "docker-rust"

RUN apt update && apt install -y --no-install-recommends curl build-essential ca-certificates libssl-dev pkg-config
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN /root/.cargo/bin/rustup default nightly

# Prepare to build
RUN mkdir /rust
WORKDIR /rust
COPY . /rust/
RUN rm -rf target

# build
RUN /root/.cargo/bin/cargo build --release

CMD target/release/$NAMESPACE


EXPOSE 3000