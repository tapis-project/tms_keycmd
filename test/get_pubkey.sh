#!/bin/bash
#
# Get public key given a json file for the request body
#
#KEY_JSON="get_key_testhostaccount1.json"
KEY_JSON="$1"
set -xv
curl -k -X POST -H "content-type: application/json" \
     https://129.114.35.127:3001/v1/tms/pubkeys/creds/retrieve -d @${KEY_JSON} | jq
