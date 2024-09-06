#!/bin/bash
#
set -xv
curl -k -X POST -H 'content-type: application/json' \
	https://129.114.35.127:3001/v1/tms/client -d @create_client_req.json | jq
