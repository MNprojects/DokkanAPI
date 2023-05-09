# Use Node.js v14 as base image
FROM node:14 AS node-app

WORKDIR /
# Copy package.json and package-lock.json to container
COPY package*.json ./
# Install dependencies
RUN npm install
# Copy all files to container
COPY . .
# Build the GraphQL
RUN npm run build

# Rust app
FROM rust:1.54 AS rust-app
WORKDIR /
RUN apt-get update && apt-get install -y libssl-dev
COPY ./restapi .
RUN cargo build --release


# Final image
FROM alpine:latest
WORKDIR /app
COPY --from=node-app ./dist ./dist
COPY --from=rust-app /restapi/target/release/restapi ./restapi
ENV JSON_FILE=DokkanCharacterData.json
ENV PORT=1000
EXPOSE 1000
CMD ["./restapi", "&&", "npm", "start"]