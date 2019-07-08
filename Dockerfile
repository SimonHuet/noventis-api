# select image
FROM rust:1.36

# copy your source tree
COPY ./ ./

# build for release
RUN rustup default nightly
RUN cargo build --release

EXPOSE 8081:8081
# set the startup command to run your binary
CMD ["./target/release/nivantis-api"]