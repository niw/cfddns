#!/usr/bin/env bash

set -euo pipefail

if [[ -z "${API_TOKEN:-""}" ]]; then
  echo "API_TOKEN is not set" >&2
  exit 1
fi

if [[ -z "${ZONE_ID:-""}" ]]; then
  echo "ZONE_ID is not set" >&2
  exit 1
fi

curl \
  --request GET \
  --url "https://api.cloudflare.com/client/v4/zones/${ZONE_ID}/dns_records" \
  --header "Authorization: Bearer ${API_TOKEN}" \
  --header "Content-Type: application/json"
