FROM rust:1.78 as rustBuilder

COPY ./server .
RUN cargo --version && rustc --version
RUN pwd && ls -la && cargo build --release


# Section for the build
# FROM cimg/rust:1.78.0-node AS build
FROM node:bookworm-slim AS nodeBuilder
# WORKDIR /app

# Setup the working directory
COPY ./client .
# ENV PATH /app/client/node_modules/.bin:$PATH

RUN pwd && ls -la && npm install && npm run build && npm run clean
# Build the project & clean
# RUN pwd && ls -la && cd ./server && cargo clean && ls -la && cargo build --release
# RUN cd ./client && npm install && npm run build && npm run clean

# Section for the final image
FROM node:bookworm-slim

# Copy complete builds from the previous stage
COPY --from=nodeBuilder build ./build
COPY --from=rustBuilder target/release/server ./server

# Install serve for serving the client
RUN npm install -g serve

# Expose the ports
EXPOSE 8080
EXPOSE 3000

# Execution
CMD ./server & serve -p 3000 -s ./build
