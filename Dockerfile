# Use the cargo-lambda image for building
FROM ghcr.io/cargo-lambda/cargo-lambda:latest as builder

# Create a directory for your application
WORKDIR /usr/src/app

# Copy your source code into the container
COPY . .

# Build the Lambda function using cargo-lambda
RUN cargo lambda build --release --arm64

# Use a new stage for the final image
# copy artifacts to a clean image
FROM public.ecr.aws/lambda/provided:al2-arm64

# Create a directory for your lambda function
WORKDIR /mini_project10

# Copy the bootstrap binary from the builder stage
COPY --from=builder /usr/src/app/target/ ./ 
# Copy the llama model here 
COPY --from=builder /usr/src/app/src/pythia-1b-q4_0-ggjt.bin ./ 

# Check to make sure files are there 
RUN if [ -d /mini_project10/lambda/transformer/ ]; then echo "Directory '/mini_project10' exists"; else echo "Directory '/mini_project10' does not exist"; fi
RUN if [ -f /mini_project10/lambda/transformer/bootstrap]; then echo "File '/mini_project10/lambda/lambda/bootstrap' exists"; else echo "File '/mini_project10/lambda/lambda/bootstrap' does not exist"; fi

# Set the entrypoint for the Lambda function
ENTRYPOINT ["/mini_project10/lambda/transformer/bootstrap"]