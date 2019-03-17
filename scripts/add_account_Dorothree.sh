#!/bin/bash

curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "add_account", 
"params": ["base_topic","Dorothee","0267d89705329e8c5b357ee2221b71d1a00443cb7fd4c16f96af17ccdfbd62d1d0"],
"id" : 1
}'


echo
