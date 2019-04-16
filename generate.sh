#!/bin/bash -eux

this_dir="$(cd "$(dirname $0)" && pwd )"

rm -rf src docs Cargo.*

openapi-generator generate \
  --skip-validate-spec \
  -i "$this_dir/ridb.yaml" \
  -o "$this_dir" \
  -g rust \
  -D packageName=ridb-client \
  --library=reqwest \
  $@

