#!/bin/bash
#
# Create a keypair given a json file for the request body
#
#KEY_JSON="test_pubkey_testhostaccount1.json"
KEY_JSON="$1"
set -xv
curl -k -X POST -H "content-type: application/json" \
	        -H "X-TMS-CLIENT-ID: testclient1" \
	        -H "X-TMS-CLIENT-SECRET: secret1" \
	        -H "X-TMS-TENANT: test" \
		https://129.114.35.127:3001/v1/tms/pubkeys/creds \
     -d @${KEY_JSON} | jq
