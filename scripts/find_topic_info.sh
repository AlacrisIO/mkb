#!/bin/bash

echo "OPER: Creating a topic"


curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8806 --data '{
"jsonrpc": "2.0", 
"method": "find_topic_info", 
"params": ["base_topic"],
"id" : 1
}'


echo
