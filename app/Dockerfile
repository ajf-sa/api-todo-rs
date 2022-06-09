FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin rust-docker-web
WORKDIR ./rust-docker-web
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/app*
RUN cargo build --release


FROM alpine:latest

ARG APP=/usr/src/app



ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/rust-docker-web/target/x86_64-unknown-linux-musl/release/app ${APP}/app
COPY --from=builder /home/rust/src/rust-docker-web/src/static ${APP}/static
COPY --from=builder /home/rust/src/rust-docker-web/src/.env ${APP}/.env

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./app"]