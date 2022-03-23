FROM rust:1.59.0 as build-image

WORKDIR /rust/hello_world
COPY src/ /rust/hello_world/src/
COPY Cargo.toml /rust/hello_world/

RUN rustup update && \
    rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM public.ecr.aws/lambda/provided:al2

COPY  --from=build-image /rust/hello_world/target/x86_64-unknown-linux-musl/release/bootstrap ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD [ "lambda-handler" ]