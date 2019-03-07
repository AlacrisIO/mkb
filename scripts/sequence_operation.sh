#!/bin/bash

echo "OPER: Creating a topic"


curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "topic_creation", 
"params": ["base_topic", 0, 0, 0, 0, "Keccak256"],
"id" : 1
}'

echo "OPER: Doing add_account operation"

curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "add_account", 
"params": ["base_topic","Dorothee","0267d89705329e8c5b357ee2221b71d1a00443cb7fd4c16f96af17ccdfbd62d1d0","8a4fa12b34f4a72bfbd4e2db90dbab1aa531aaadb41d1592547f72901350dc78"],
"id" : 1
}'


echo
