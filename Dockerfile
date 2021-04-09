FROM brossquad/fiber-dev:1.0.4 as builder

ARG ENVIRONMENT=development
ARG RACE=0
ARG VERSION=dev

COPY . /app

WORKDIR /app


RUN go mod download \
    && make test RACE=${RACE} \
    && make clean \
    && make build RACE=0 VERSION=${VERSION} ENVIRONMENT=${ENVIRONMENT} -j8


FROM alpine:3.13 as runner

COPY --from=builder /app/bin/hosts /usr/bin/hosts
