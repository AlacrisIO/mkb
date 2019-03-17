#!/bin/bash

echo "OPER: Creating a topic"


curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "send_data", 
"params": ["base_topic", "Dorothee", [], "test_string for record"],
"id" : 1
}'


echo
