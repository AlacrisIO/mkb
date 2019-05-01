#!/bin/bash

#

echo "OPER 1: Creating a topic"
curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "topic_creation", 
"params": ["base_topic", 4, 0, 0, 0, 0, 0, 0, 0, "Keccak256"],
"id" : 1
}'

# ANSWER IS
# {"jsonrpc":"2.0","result":"successful answer, nothing to report","id":1}



echo "OPER 2: add a registrar"
curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "add_registrar", 
"params": ["base_topic", "0x9ccaed210ce8c0cb49c5ad1c4f583406c264ba69"],
"id" : 1
}'

# ANSWER IS
# {"jsonrpc":"2.0","result":"successful answer, nothing to report","id":1}



echo "OPER 3: Doing add_account operation"
curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "add_account", 
"params": ["base_topic","Dorothee","0267d89705329e8c5b357ee2221b71d1a00443cb7fd4c16f96af17ccdfbd62d1d0"],
"id" : 1
}'

# ANSWER IS
# {"jsonrpc":"2.0","result":"{\"Mkboperation\":{\"signature\":[]}}","id":1}


echo "OPER 4: get_from_latest"
curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{
"jsonrpc": "2.0", 
"method": "get_from_latest", 
"params": ["base_topic", "Dorothee"],
"id" : 1
}'

# ANSWER IS
#

echo
