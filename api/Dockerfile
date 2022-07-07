FROM rust:1.61 as builder
WORKDIR /usr/src/api
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get -y install libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["api"]%    