FROM rust:latest
MAINTAINER caleb <calebsmithwoolrich@gmail.com>

RUN echo "Version: 0.1.29"

EXPOSE 8000
EXPOSE 56700
EXPOSE 56700/udp

RUN mkdir -p /app && git clone https://github.com/PixelCoda/lifx-api-server /app \
    && cd /app \
    && cargo build --release  \
    && rm -Rf /app/src  /app/target/release/build /app/target/release/deps /app/target/release/examples/ /app/target/release/incremental/ /app/target/release/native
WORKDIR /app/target/release
CMD ["./lifx-api-server"]