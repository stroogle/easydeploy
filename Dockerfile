FROM rust:1.82.0-alpine AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM alpine:3.21.3

WORKDIR /app

COPY --from=builder /app/target/release/easydeploy .
COPY ./cron.sh /app/cron.sh

RUN apk update
RUN apk add docker-cli-compose
RUN apk add bash

RUN chmod +x /app/cron.sh

ENV TIMING="* * * * *"

CMD printf "${TIMING} /bin/bash /app/cron.sh >> /tmp/log 2>&1\n\n" > /app/cron-job && crontab /app/cron-job && crond -l 2 -f 
