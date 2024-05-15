FROM rust:1.78 as rustBuilder

COPY ./server .

RUN cargo build --release


FROM node:bookworm-slim AS nodeBuilder

COPY ./client .

RUN npm install && npm run build && npm run clean

# Final image
FROM node:bookworm-slim

# Copy executables
COPY --from=nodeBuilder build ./build
COPY --from=rustBuilder target/release/server ./server

# Install serve for serving the client
RUN npm install -g serve

# Expose the ports
EXPOSE 8080
EXPOSE 3000

# Execution
CMD ./server & serve -p 3000 -s ./build &
