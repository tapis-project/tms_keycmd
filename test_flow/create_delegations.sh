#!/bin/bash
#
REQ_JSON=create_delegations_req.json
set -xv
curl -X POST -H "content-type: application/json" \
       	-H "X-TMS-TENANT: $TMS_TENANT" \
       	-H "X-TMS-ADMIN-ID: $TMS_ADMIN_ID" \
       	-H "X-TMS-ADMIN-SECRET: $TMS_ADMIN_KEY" \
	$TMS_URL/v1/tms/delegations -d @$REQ_JSON
