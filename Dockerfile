# Section for the build
FROM cimg/rust:1.78.0-node AS build
# WORKDIR /app

# Setup the working directory
COPY . .
# ENV PATH /app/client/node_modules/.bin:$PATH

# Build the project & clean
RUN pwd && ls -la && cd ./server && cargo clean && ls -la && cargo build --release
RUN cd ./client && npm install && npm run build && npm run clean

# Section for the final image
FROM node:bookworm-slim

# Copy complete builds from the previous stage
COPY --from=build /home/circleci/project/client/build ./build
COPY --from=build /home/circleci/project/server/target/release/server ./server

# Install serve for serving the client
RUN npm install -g serve

# Expose the ports
EXPOSE 8080
EXPOSE 3000

# Execution
CMD ./server & serve -p 3000 -s ./build
