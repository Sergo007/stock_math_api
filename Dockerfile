# https://hub.docker.com/layers/library/alpine/3.19.1/images/sha256-6457d53fb065d6f250e1504b9bc42d5b6c65941d57532c072d929dd0628977d0?context=explore
FROM alpine:3.20.2@sha256:0a4eaa0eecf5f8c050e5bba433f58c052be7587ee8af3e8b3910ef9ab5fbe9f5 AS base
ENV TERM xterm-256color
RUN apk add --no-cache rust cargo pkgconfig openssl-dev
WORKDIR /builder
COPY . .

#
# builder state
#
FROM base AS builder
RUN cargo install --path . --root .

#
# test stage
#
FROM base AS test
RUN apk add --no-cache openrc

CMD cd /builder && \
    cargo test

#
# create clean image with chat application only
#
FROM alpine:3.20.2@sha256:0a4eaa0eecf5f8c050e5bba433f58c052be7587ee8af3e8b3910ef9ab5fbe9f5 AS runtime
# libgcc is required to run rust applications
RUN apk add --no-cache libgcc

COPY --from=builder /builder/bin/* /usr/local/bin
COPY ui-admin /usr/local/bin/ui-admin
RUN adduser user -D -G users
USER user

EXPOSE 8080

WORKDIR /usr/local/bin
ENTRYPOINT ["/usr/local/bin/stock_math_api"]
