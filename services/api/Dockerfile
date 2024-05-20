 # Build app
FROM rust:1.76.0-buster as builder

# create a new empty shell project

RUN USER=root cargo new --bin api
WORKDIR /api

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/api*
RUN cargo build --release

# our final base
FROM rust:1.76.0-buster
RUN apt-get update && apt install -y openssl


# copy the build artifact from the build stage
COPY --from=builder /api/target/release/api .


# set the startup command to run your binary
CMD ["./api"]
