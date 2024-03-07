FROM mcr.microsoft.com/devcontainers/rust:1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends postgresql-client \
    && cargo install sea-orm-cli cargo-insta \
    && chown -R vscode /usr/local/cargo

# Install sea-orm-cli
RUN cargo install sea-orm-cli

# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y nodejs
# Install Angular CLI
RUN npm install -g @angular/cli