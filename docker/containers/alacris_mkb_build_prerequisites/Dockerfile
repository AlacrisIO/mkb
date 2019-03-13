FROM debian:stretch

WORKDIR /root

# Add non-root user
RUN groupadd -g 1100 appuser && \
    useradd -r -u 1100 -g appuser -m -d /home/appuser appuser

# common packages
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    ca-certificates curl file wget gnupg2 software-properties-common \
    build-essential \
    autoconf automake autotools-dev libtool xutils-dev && \
    rm -rf /var/lib/apt/lists/*

# install clang
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key| apt-key add - && \
    apt-add-repository 'deb http://apt.llvm.org/stretch/ llvm-toolchain-stretch main' && \
    apt-get update && apt-get install -y  clang-3.8 lldb-3.8

ENV SSL_VERSION=1.0.2q

# install openssl
RUN curl https://www.openssl.org/source/openssl-$SSL_VERSION.tar.gz -O && \
    tar -xzf openssl-$SSL_VERSION.tar.gz && \
    cd openssl-$SSL_VERSION && ./config && make depend && make install && \
    cd .. && rm -rf openssl-$SSL_VERSION*

ENV OPENSSL_LIB_DIR=/usr/local/ssl/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/ssl/include \
    OPENSSL_STATIC=1

USER appuser
WORKDIR /home/appuser

ENV RUST_VERSION=1.32.0
# install toolchain
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain $RUST_VERSION -y

ENV PATH=/home/appuser/.cargo/bin:$PATH
