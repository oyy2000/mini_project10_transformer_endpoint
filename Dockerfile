# Use an official Rust image
FROM rust:1.75 as builder

RUN apt-get update
RUN apt-get install -y build-essential cmake curl openssl libssl-dev

ARG LIBTORCH_URL=https://download.pytorch.org/libtorch/cu118/libtorch-cxx11-abi-shared-with-deps-2.0.0%2Bcu118.zip

RUN curl -L ${LIBTORCH_URL} -o libtorch.zip && \
    unzip libtorch.zip -d / && \
    rm libtorch.zip

ENV LIBTORCH=/libtorch
ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

# Create a new empty shell project
RUN USER=root cargo new mini10
WORKDIR /mini10

# Copy the manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This is a dummy build to get the dependencies cached
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependencies are built, copy your source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/mini10*
RUN cargo build --release

# test
RUN cargo test -v

# Final stage
FROM debian:bookworm-slim
RUN apt-get update
RUN apt-get install -y build-essential cmake curl openssl libssl-dev
COPY --from=builder /mini10/target/release/mini10 .
COPY --from=builder /libtorch/ /libtorch/
ENV LIBTORCH=/libtorch
ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["./mini10"]

