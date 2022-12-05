FROM rust:alpine as prepare-stage
WORKDIR /app
COPY src src
COPY Cargo.toml Cargo.toml
COPY .cargo .cargo
COPY vendor vendor

FROM prepare-stage as build-stage
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM rust:alpine
EXPOSE 8000
ENV TZ=Pacific/Auckland \
    USER=staff
RUN addgroup -S $USER \
    && adduser -S -g $USER $USER
RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

WORKDIR /app
COPY --from=build-stage /app/target/release/remote_hut remote_hut
COPY Rocket.toml .
COPY static static
COPY src/views src/views

RUN mkdir logs

RUN chown -R $USER:$USER /app

USER $USER
CMD ["./remote_hut"]
