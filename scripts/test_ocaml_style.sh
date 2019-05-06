#!/bin/bash

#
echo "OPER 1: Creating a topic"
curl -s -X POST -H 'Content-Type: application/json' -i http://127.0.0.1:8805 --data '{"jsonrpc": "2.0","method": "topic_creation_test","params":{"topic":"base_topic", "committee_size_str":"0x01"},"id":1}'
