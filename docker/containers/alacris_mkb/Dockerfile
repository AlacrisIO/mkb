FROM gcr.io/legicash-demo-1950/legicash-demo/alacris-mkb-build-prerequisites:v1 AS build

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
FROM gcr.io/legicash-demo-1950/legicash-demo/alacris_mkb_run_prerequisites:v1

# copy keygen
COPY --from=build \
     /var/www/app/legicash-mkb/target/release/keygen_secp256k1 /var/www/app/legicash-mkb/keygen_secp256k1

# copy registrar
COPY --from=build \
     /var/www/app/legicash-mkb/target/release/mkb_registrar /var/www/app/legicash-mkb/mkb_registrar

# Install supervisor config
#ADD docker/containers/alacris_mkb/files/conf/supervisord.conf /etc/supervisord.conf

# Service scripts
#ADD docker/containers/alacris_mkb/files/scripts/run_registrar_bob.sh /usr/local/bin/run_registrar_bob
#ADD docker/containers/alacris_mkb/files/scripts/run_registrar_alice.sh /usr/local/bin/run_registrar_alice

COPY docker/containers/alacris_mkb/files/scripts/keygen.sh /keygen.sh
USER root
RUN chmod +x /keygen.sh

USER appuser

ENTRYPOINT ["/keygen.sh"]

EXPOSE 8805 8806

# command when running image
CMD ["/usr/bin/supervisord", "-n", "-c", "/etc/supervisord.conf"]