FROM gcr.io/legicash-demo-1950/legicash-demo/alacris-mkb-build-preprequisites:latest

# create a new empty shell project
RUN USER=root cargo new --bin empty-project
WORKDIR /empty-project

# copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy  source tree
COPY ./src ./var/www/app/legicash-mkb

WORKDIR /var/www/app/legicash-mkb

# build for release
RUN rm ./target/release/deps/empty_project*
RUN cargo build --release