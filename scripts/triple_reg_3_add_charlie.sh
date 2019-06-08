#!/bin/bash

echo "OPER: Creating a topic"


curl -s -X POST -H 'Content-Type: application/json' -i http://localhost:8805 --data '{
"jsonrpc": "2.0",
"method": "add_registrar",
"params": ["mkb_side_chain_registry", "Charlie"],
"id" : 1
}'


echo
