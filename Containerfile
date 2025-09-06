FROM rustlang/rust:nightly-trixie AS builder

# Download cargo-leptos as binary without building it
# TODO: Figure out a way to track `cargo-leptos` for updates.
RUN curl --proto '=https' --tlsv1.2 --location --silent --show-error --fail https://github.com/leptos-rs/cargo-leptos/releases/download/v0.2.43/cargo-leptos-installer.sh | sh

WORKDIR /app
COPY . .

# Install the required toolchain and components
RUN rustc --version

RUN cargo fetch -vv

# Build the application with its utilities
RUN cargo leptos build --release --split -vv && \
cargo build --package migration --release -vv && \
cargo build --package jidhom_control --release -vv


FROM debian:trixie-slim AS runtime

COPY --from=builder /app/target/release/jidhom_control /app/
COPY --from=builder /app/target/release/migration /app/
COPY --from=builder /app/target/release/jidhom /app/
COPY --from=builder /app/target/site /app/site

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

CMD ["/app/jidhom"]
