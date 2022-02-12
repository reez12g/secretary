# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --release

EXPOSE 3030

# Run the web service on container startup.
ENTRYPOINT ["target/release/secretary"]
