FROM rust:1.45 as builder
WORKDIR .
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder . /usr/local/bin/discord-shill-bot
CMD ["/usr/local/bin/discord-shill-bot"]