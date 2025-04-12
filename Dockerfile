FROM rust:1.82.0 as builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM docker:28.1.0-rc.1-cli-alpine3.21

WORKDIR /app

COPY --from=builder /app/target/release/easydeploy .

CMD [ "./easydeploy" ]