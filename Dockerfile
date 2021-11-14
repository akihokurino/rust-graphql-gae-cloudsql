FROM rust:1.53.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:1.53.0 AS deploy
WORKDIR /app
COPY --from=builder /app/target/release/api api
ENV TZ=Asia/Tokyo
EXPOSE 8080
CMD /app/api