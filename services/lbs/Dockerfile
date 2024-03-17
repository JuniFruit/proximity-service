# Build app
FROM rust:1.76.0-buster as builder
# create a new empty shell project
RUN USER=root cargo new --bin lbs
WORKDIR /lbs

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/lbs*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=builder /lbs/target/release/lbs .

# set the startup command to run your binary
CMD ["./lbs"]
