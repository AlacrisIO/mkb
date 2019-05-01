#!/bin/bash

echo "OPER: Creating a topic"


curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "add_registrar", 
"params": ["base_topic", "0x9ccaed210ce8c0cb49c5ad1c4f583406c264ba69"],
"id" : 1
}'


echo
