FROM rust:1.82.0-alpine AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

#FROM docker:28.1.0-rc.1-cli-alpine3.21

FROM alpine:3.21.3

WORKDIR /app

COPY --from=builder /app/target/release/easydeploy .

RUN apk update
RUN apk add docker-cli-compose

RUN crontab -l | { cat; echo "0 * * * * flock -n /app/easydeploy.lock /app/easydeploy >> /var/log/cron.log"; } | crontab -

RUN touch /var/log/cron.log

CMD crond && tail -f /var/log/cron.log