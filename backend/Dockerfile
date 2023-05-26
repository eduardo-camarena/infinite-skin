FROM rust:1.67.0

WORKDIR /app

RUN apt-get install libmariadb-dev-compat libmariadb-dev
COPY Cargo.lock Cargo.toml ./
COPY src/ src/
RUN cargo build
RUN cargo install cargo-watch

CMD cargo watch -x run
