#!/bin/bash

echo "OPER: Creating a topic"


curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0",
"method": "topic_creation",
"params": {"topic":"mkb_side_chain_registry","committee_size":0,"min_interval_insertion_micros":0,"total_capacity_mem":0,"instant_capacity_mem":0,"total_throughput_per_min":0,"total_throughput_per_sec":0,"retention_time":0,"retention_size":0,"hash_method":"Keccak256"},
"id" : 1
}'


echo "OPER: Creating a topic"


curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0",
"method": "add_registrar",
"params": ["mkb_side_chain_registry", "Bob"],
"id" : 1
}'


echo "OPER: Creating an account"


curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0",
"method": "add_account",
"params": ["mkb_side_chain_registry","John"],
"id" : 1
}'


echo "OPER: Send data"

curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0",
"method": "send_data",
"params": ["mkb_side_chain_registry","John","0x0000000000000000000000000000000000000000000000000000000000000000","0xf9851cf6060d48301f05a962b4b8667e12a9c85894d3433cab86a8c5f70dbf97"],
"id" : 1
}'

echo
