#!/bin/bash
set -e

cd /var/www/app/legicash-mkb && ./keygen_secp256k1  ./config/common_init_file.json

exec "$@"