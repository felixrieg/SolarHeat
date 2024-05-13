FROM cimg/rust:1.78.0-node

# Setup the working directory
COPY . .
ENV PATH /app/client/node_modules/.bin:$PATH
RUN npm install -g serve

# Build the project
RUN cd ./server && pwd && cargo build --release
RUN cd ./client && npm install && npm run build

# Expose the ports
EXPOSE 8080
EXPOSE 3000

# Run the project
CMD ./server/target/release/server & serve -p 3000 -s ./client/build
