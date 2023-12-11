FROM rust:1.67-buster as builder

# install protobuf
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

COPY Cargo.toml build.rs /usr/src/app/
COPY src /usr/src/app/src/
COPY library /usr/src/app/library/
COPY proto /usr/src/app/proto/
WORKDIR /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --bin movies-back

FROM gcr.io/distroless/static-debian11 as runner

# get binary
COPY --from=builder /usr/src/app/target/release/movies-back /

# set run env
EXPOSE 50051

# run it
#CMD ["/movies-back"]