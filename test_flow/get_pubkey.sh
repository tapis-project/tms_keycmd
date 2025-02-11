#!/bin/bash
#
#REQ_JSON=get_keypair_req.json
# Test tenant=test, client=testclient1, secret=secret
REQ_JSON=$1
set -xv
curl -k -X POST -H "content-type: application/json" \
	$TMS_URL/v1/tms/pubkeys/creds/retrieve -d @$1
