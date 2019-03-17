#!/bin/bash

echo "OPER: Creating a topic"


curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "send_data", 
"params": ["base_topic", "Dorothee", [27,32,61,40,156,244,28,128,195,193,253,209,15,97,90,236,4,148,49,62,225,216,101,217,111,109,130,118,130,193,229,143,249,36], "test_string for record"],
"id" : 1
}'


echo
