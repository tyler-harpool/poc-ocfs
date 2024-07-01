FROM rust:slim AS builder

RUN rustup target add wasm32-unknown-unknown

RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /usr/src/app

# copy entire workspace
COPY . .

RUN cargo build --release


FROM alpine
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/docket_api ./
CMD [ "./docket_api" ]
LABEL service=docket_api

FROM alpine
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/judges_api ./
CMD [ "./juges_api" ]
LABEL service=judges_api

FROM alpine
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/ui ./
CMD [ "./ui" ]
LABEL service=ui

FROM alpine
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/gateway_api ./
CMD [ "./gateway_api" ]
LABEL service=gateway_api