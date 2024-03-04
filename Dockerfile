FROM mcr.microsoft.com/vscode/devcontainers/rust:1-1-bullseye

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
     && apt-get -y install --no-install-recommends postgresql-client \
     && apt install build-essential \
     && chown -R vscode /usr/local/cargo

# Install sea-orm-cli
RUN cargo install sea-orm-cli

# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
     && apt-get install nodejs -y

# Install Angular CLI
RUN npm install -g @angular/cli

# Set the working directory
WORKDIR /workspaces/app

COPY . .

RUN cd /workspaces/app/frontend \
     && npm install \
     && ng build
