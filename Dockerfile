FROM rust:1.73.0 as builder
WORKDIR /usr/lib/x3dh
COPY ./lib/x3dh .
WORKDIR /usr/src/server
COPY ./application/plasma-server .
RUN cargo install --path .

FROM debian:bookworm-20231009-slim
COPY --from=builder /usr/local/cargo/bin/plasma-server /usr/local/bin/plasma-server
COPY --from=builder /usr/src/server/.env.docker /.env
CMD ["plasma-server"]
