# Use the official Rust image as the base image
FROM rust:1.65

RUN apt-get update && \
    apt-get -y install libssl-dev openssl zlib1g zlib1g-dev libpq-dev cmake protobuf-compiler netcat curl unzip

# Set the working directory inside the container
WORKDIR /app

# Copy the project's Cargo.toml and Cargo.lock files into the container
COPY ./kube/playground/Cargo.toml ./kube/playground/Cargo.lock ./

COPY ./kube/playground/src /app/src

# Build the project's dependencies
RUN cargo build

CMD ["tail", "-f", "/dev/null"]