# Stage 1: Build the Angular application
FROM node:latest as angular-build

# Set the working directory for the Angular app
WORKDIR /app

# Copy the Angular app files to the container
COPY web_client/ /app/

# Install npm dependencies for Angular
RUN npm install

# Build the Angular app
RUN npm run build

# Stage 2: Build the Rust application
FROM rust:latest as rust-build

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Rust project files to the container
COPY . .

# Copy the built Angular app from the previous stage
COPY --from=angular-build /app/dist/ /usr/src/app/web_client/dist/

# Build the Rust application
RUN cargo build --release

# Stage 3: Create the final image
FROM ubuntu:latest

# Install needed runtime libraries, including OpenSSL
RUN apt-get update && \
	apt-get install -y --no-install-recommends \
	ca-certificates \
	libssl3 \
	&& rm -rf /var/lib/apt/lists/*


# Set the working directory in the container
WORKDIR /app

# Copy the built Rust binary
COPY --from=rust-build /usr/src/app/target/release/kratos-rs /app/

# Copy the built Angular app
COPY --from=rust-build /usr/src/app/web_client/dist/ /app/web_client/dist/

# Expose the port the app runs on
EXPOSE 3000

# Command to run the binary
CMD ["/app/kratos-rs"]

