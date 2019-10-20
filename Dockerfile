# define ubuntu version, you can use --build-arg
ARG ubuntu_version="19.10"
FROM ubuntu:${ubuntu_version}

# Dockerfile on bash
SHELL ["/bin/sh", "-c"]

EXPOSE 3000