FROM gcr.io/legicash-demo-1950/legicash-demo/alacris_mkb_build_prerequisites:v1 AS dependencies

# Create dummy project
RUN USER=root cargo new --bin dummy_project

WORKDIR /dummy_project

# copy over dummy files
COPY docker/containers/alacris_mkb/dummy_src/ .

COPY ./Cargo.toml .

USER root

# Set permissions for appuser
RUN mkdir -p /var/www/app/legicash-mkb/ && chown -R appuser:appuser /var/www/app/legicash-mkb/ /dummy_project

USER appuser

# this build step will cache dependencies
RUN cargo build --release

RUN rm src/*.rs

# Copy source tree
COPY src/ src/

# build for release
RUN rm ./target/release/deps/mkb_*

RUN cargo build --release

# create runtime image
FROM gcr.io/legicash-demo-1950/legicash-demo/alacris_mkb_run_prerequisites:v1

# copy registrar
COPY --from=dependencies \
     /dummy_project/target/release/mkb_registrar /var/www/app/legicash-mkb/mkb_registrar

USER appuser


EXPOSE 8805 8806

# command when running image
CMD ["/usr/bin/supervisord", "-n", "-c", "/etc/supervisord.conf"]