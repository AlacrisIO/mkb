FROM gcr.io/legicash-demo-1950/legicash-demo/alacris-mkb-build-preprequisites:v1 AS build

# copy over dummy files
COPY docker/containers/alacris_mkb/dummy_src/ /var/www/app/legicash-mkb/

WORKDIR /var/www/app/legicash-mkb/

COPY ./Cargo.toml .

# Set permissions for appuser
RUN chown -R appuser:appuser /var/www/app/legicash-mkb/

#USER appuser

# this build step will cache dependencies
RUN cargo build --release --verbose

# copy  source tree
ADD . .

# build for release
RUN cargo build --release
FROM gcr.io/legicash-demo-1950/legicash-demo/alacris-mkb-build-preprequisites:latest
COPY --from=build \
     /var/www/app/legicash-mkb/target/release/keygen_secp256k1 /var/www/app/legicash-mkb/target/release/keygen_secp256k1