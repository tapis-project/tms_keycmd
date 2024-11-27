#!/bin/bash
#
set -xv
curl -k -X POST -H 'content-type: application/json' \
	$TMS_URL/v1/tms/client -d @create_client_tapis_req.json | jq
