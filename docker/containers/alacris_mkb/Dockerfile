FROM gcr.io/legicash-demo-1950/legicash-demo/alacris-mkb-build-prerequisites:v1 AS dependencies

# copy over dummy files
COPY docker/containers/alacris_mkb/dummy_src/ /var/www/app/dummy-mkb/

WORKDIR /var/www/app/dummy-mkb/

COPY ./Cargo.toml .

USER root
# Set permissions for appuser
RUN mkdir -p /var/www/app/legicash-mkb/ && chown -R appuser:appuser /var/www/app/legicash-mkb/ /var/www/app/dummy-mkb/

#USER appuser

# this build step will cache dependencies
#RUN cargo build --release
RUN rustup default nightly-2019-01-29 && \
    cargo build -Z unstable-options --out-dir /output

#RUN rm -rf /var/www/app/legicash-mkb/target/*

FROM gcr.io/legicash-demo-1950/legicash-demo/alacris-mkb-build-prerequisites:v1 AS application

WORKDIR /var/www/app/legicash-mkb/

COPY --from=dependencies /var/www/app/dummy-mkb/Cargo.toml .
#COPY --from=dependencies /usr/local/cargo /usr/local/cargo

# copy source tree
#COPY . .
COPY src/ src/
VOLUME /output

USER root
# build for release
#RUN cargo build --release
RUN rustup default nightly-2019-01-29  && \
    cargo build --release -Z unstable-options --out-dir /output

# create runtime image
FROM gcr.io/legicash-demo-1950/legicash-demo/alacris_mkb_run_prerequisites:v1

# copy registrar
COPY --from=application \
     /var/www/app/legicash-mkb/target/release/mkb_registrar /var/www/app/legicash-mkb/mkb_registrar

USER appuser


EXPOSE 8805 8806

# command when running image
CMD ["/usr/bin/supervisord", "-n", "-c", "/etc/supervisord.conf"]