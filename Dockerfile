FROM rust:1.74-slim as builder

RUN apt-get update \
    && apt-get install -y \
    curl

# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install nodejs -y

# Install Angular CLI
RUN npm install -g @angular/cli

WORKDIR /usr/src/

COPY . .

RUN cd /usr/src/frontend \
    && npm install \
    && ng build

RUN cargo build --release

FROM debian:bookworm-slim

ARG APP_NAME=lc_xrp

WORKDIR /usr/app

COPY --from=builder /usr/src/frontend/dist/frontend/browser /usr/app/frontend/dist/frontend/browser
COPY --from=builder /usr/src/frontend/dist/frontend/browser/index.html /usr/app/frontend/dist/frontend/browser/index.html
COPY --from=builder /usr/src/assets /usr/app/assets
COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/lc_xrp-cli /usr/app/lc_xrp-cli

ENTRYPOINT ["/usr/app/lc_xrp-cli", "start", "-e", "production"]