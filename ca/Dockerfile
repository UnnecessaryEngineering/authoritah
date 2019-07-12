FROM alpine:latest as cargo-build
RUN apk --no-cache add mariadb-connector-c mariadb-dev rust cargo openssl-dev

WORKDIR /usr/src/authoritah/lib
COPY lib/Cargo.toml Cargo.toml
COPY lib/src src

WORKDIR /usr/src/authoritah/ca
COPY ca/Cargo.toml Cargo.toml
COPY ca/src src

RUN cargo build --release

FROM alpine:latest
COPY --from=cargo-build /usr/src/authoritah/ca/target/release/authoritah-ca /usr/local/bin/authoritah-ca
RUN apk --no-cache add mariadb-connector-c libgcc

CMD ["authoritah-ca"]