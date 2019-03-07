FROM rust:1.32.0

WORKDIR /var/www/app/legicash-mkb
COPY . .
RUN rustc --version
RUN RUST_BACKTRACE=1 cargo build --verbose