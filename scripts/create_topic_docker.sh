#!/bin/bash

echo "OPER: Creating a topic"


curl -s -X POST -H 'Content-Type: application/json' -i http://alacris-registrar-alice:8805 --data '{
"jsonrpc": "2.0",
"method": "topic_creation",
"params": {"topic":"mkb_side_chain_registry","committee_size":0,"min_interval_insertion_micros":0,"total_capacity_mem":0,"instant_capacity_mem":0,"total_throughput_per_min":0,"total_throughput_per_sec":0,"retention_time":0,"retention_size":0,"hash_method":"Keccak256"},
"id" : 1
}'


echo
