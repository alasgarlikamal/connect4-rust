# Start with a rust alpine image
FROM rust:1-alpine3.19

# set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app

# do a release build
RUN cargo build --release
RUN strip target/release/connectfour

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.19

# copy the binary into the final image
COPY --from=0 /app/target/release/connectfour .

# set the binary as entrypoint
ENTRYPOINT ["/connectfour"]
