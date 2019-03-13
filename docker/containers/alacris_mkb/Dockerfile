FROM gcr.io/legicash-demo-1950/legicash-demo/alacris-mkb-build-preprequisites:v1 AS build

# copy over dummy files
COPY docker/containers/alacris_mkb/dummy_src/ /var/www/app/dummy-mkb/

WORKDIR /var/www/app/dummy-mkb/

COPY ./Cargo.toml .

USER root
# Set permissions for appuser
RUN mkdir -p /var/www/app/legicash-mkb/ && chown -R appuser:appuser /var/www/app/legicash-mkb/ /var/www/app/dummy-mkb/

USER appuser
# this build step will cache dependencies
RUN cargo build --release

#RUN rm -rf /var/www/app/legicash-mkb/target/*

WORKDIR /var/www/app/legicash-mkb/
# copy source tree
ADD . .

# build for release
RUN cargo build --release

# create runtime image
FROM gcr.io/legicash-demo-1950/legicash-demo/alacris-mkb-build-preprequisites:latest
# copy keygen
COPY --from=build \
     /var/www/app/legicash-mkb/target/release/keygen_secp256k1 /var/www/app/legicash-mkb/target/release/keygen_secp256k1

# copy registrar
COPY --from=build \
     /var/www/app/legicash-mkb/target/release/keygen_secp256k1 /var/www/app/legicash-mkb/target/release/keygen_secp256k1
