#!/bin/bash
#
set -xv
curl -k $TMS_URL/v1/tms/version | jq
