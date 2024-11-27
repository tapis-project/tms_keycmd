#!/bin/bash
#
# Create a keypair given a json file for the request body
#
#KEY_JSON="test_pubkey_testhostaccount1.json"
#BASE_URL=https://129.114.35.127:3001
#CLIENT_ID=testclient1
#CLIENT_SECRET=secret1
BASE_URL=https://tms-server-prod.tacc.utexas.edu:3000
CLIENT_ID=testclient1
CLIENT_SECRET=secret1
KEY_JSON="$1"
set -xv
curl -X POST -H "content-type: application/json" \
	        -H "X-TMS-CLIENT-ID: $CLIENT_ID" \
	        -H "X-TMS-CLIENT-SECRET: $CLIENT_SECRET" \
	        -H "X-TMS-TENANT: test" \
		$BASE_URL//v1/tms/pubkeys/creds \
     -d @${KEY_JSON}
