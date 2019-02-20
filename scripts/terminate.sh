#!/bin/bash

curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "terminate", 
"params": [],
"id" : 1
}'


echo
