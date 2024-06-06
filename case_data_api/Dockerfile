FROM lukemathwalker/cargo-chef:latest-rust-1.72.0 as chef
WORKDIR /usr/src/app

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/case_data_api ./

CMD ["./case_data_api"]