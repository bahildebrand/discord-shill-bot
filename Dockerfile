FROM rust:1.45 as builder
WORKDIR .
COPY . .
RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/discord-shill-bot /build-out/

FROM debian:buster-slim
RUN apt-get update && apt-get -y install ca-certificates libssl-dev
COPY --from=builder /build-out/discord-shill-bot /
COPY --from=builder /log4rs.yml /
CMD /discord-shill-bot