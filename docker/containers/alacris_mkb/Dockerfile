FROM gcr.io/legicash-demo-1950/legicash-demo/alacris-mkb-build-preprequisites:latest

WORKDIR /var/www/app/legicash-mkb
COPY . .
RUN rustc --version
RUN RUST_BACKTRACE=1 cargo build # --release --verbose