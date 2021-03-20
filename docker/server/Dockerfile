FROM ubuntu:focal
MAINTAINER Mikhail Zolotukhin <mail@genda.life>

USER root
ENV DEBIAN_FRONTEND="noninteractive"

# Build utils
RUN apt-get update && apt-get install -y \
    g++ \
    make \
    cmake \
    git

# Dependencies
RUN apt-get install -y \
    libbison-dev \
    flex \
    libboost-filesystem-dev \
    zlib1g-dev

RUN git clone https://github.com/gikari/rcssserver.git --single-branch --branch cmake-port server-source
WORKDIR server-source

RUN cmake -B build -S . && cmake --build build && cmake --install build && ldconfig

ENTRYPOINT ["rcssserver"]
