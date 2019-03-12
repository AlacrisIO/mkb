#!/bin/bash

echo "OPER: Creating a topic"


curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "topic_creation", 
"params": ["base_topic", 0, 0, 0, 0, "Keccak256"],
"id" : 1
}'


echo
