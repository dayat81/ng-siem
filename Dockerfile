FROM golang:1.22-alpine AS builder

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN go build -o main .

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/main .

ENV GOOGLE_CLOUD_PROJECT=""
ENV PUBSUB_TOPIC_ID=""
ENV PUBSUB_SUBSCRIPTION_ID=""

CMD ["./main"]
