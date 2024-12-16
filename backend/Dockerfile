FROM rust:1.82.0

WORKDIR /app

RUN apt-get install libmariadb-dev-compat libmariadb-dev

COPY Cargo.lock Cargo.toml ./
COPY src/ src/
COPY entity/ entity/
COPY migration/ migration/

RUN cargo install cargo-watch

CMD cargo watch -x run
