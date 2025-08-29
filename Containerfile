FROM rustlang/rust:nightly-trixie as builder

# To download cargo-leptos as binary without building it
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz &&\
tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz &&\
cp cargo-binstall /usr/local/cargo/bin &&\
cargo binstall cargo-leptos -y

WORKDIR /app
COPY . .

# Install the required toolchain and components
RUN rustc --version

RUN cargo fetch -vv

# Build the application with its utilities
RUN cargo leptos build --release --split -vv &&\
cargo build --package migration --release -vv &&\
cargo build --package jidhom_control --release -vv


FROM debian:trixie-slim as runtime

COPY --from=builder /app/target/release/jidhom_control /app/
COPY --from=builder /app/target/release/migration /app/
COPY --from=builder /app/target/release/jidhom /app/
COPY --from=builder /app/target/site /app/site

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

CMD ["/app/jidhom"]
