#!/bin/bash
#
#curl -k -X GET https://129.114.35.127:3001/v1/tms/version | jq
set -xv
curl -X GET https://tms-server-prod.tacc.utexas.edu:3000/v1/tms/version | jq
