#!/bin/bash
#
set -xv
curl -k -X POST -H 'content-type: application/json' \
       	-H 'X-TMS-TENANT: test' \
       	-H 'X-TMS-ADMIN-ID: ~~admin' \
       	-H 'X-TMS-ADMIN-SECRET: ********************' \
	https://129.114.35.127:3001/v1/tms/userhosts -d @create_userhosts_req.json
