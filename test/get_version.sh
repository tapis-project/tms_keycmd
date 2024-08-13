#!/bin/bash
#
set -xv
curl -k -X GET https://129.114.35.127:3001/v1/tms/version | jq
