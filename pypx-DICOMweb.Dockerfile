FROM docker.io/lukemathwalker/cargo-chef:0.1.62-rust-1.72-alpine3.18 AS chef
WORKDIR /app
ARG CARGO_TERM_COLOR=always

FROM chef AS planner
COPY . .
RUN cargo chef prepare --bin pypx-DICOMweb --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --target x86_64-unknown-linux-musl --bin pypx_dicomweb --recipe-path recipe.json
COPY . .
WORKDIR /app/pypx-DICOMweb
RUN cargo build --release --locked --target x86_64-unknown-linux-musl --bin pypx_dicomweb

#FROM gcr.io/distroless/static-debian12:nonroot
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pypx_dicomweb /app/pypx_dicomweb

EXPOSE 4006
CMD ["/app/pypx_dicomweb"]
