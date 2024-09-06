#!/bin/bash
#
set -xv
curl -k -X POST -H 'content-type: application/json' \
       	-H 'X-TMS-TENANT: test' \
       	-H 'X-TMS-CLIENT-ID: tapis-client' \
       	-H 'X-TMS-CLIENT-SECRET: ********************' \
	https://129.114.35.127:3001/v1/tms/pubkeys/creds -d @create_keypair_req.json
