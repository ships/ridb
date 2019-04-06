#!/bin/bash -eux

this_dir="$(cd "$(dirname $0)" && pwd )"

openapi-generator generate \
  -i "$this_dir/ridb.yaml" \
  -o "$this_dir" \
  -g rust \
  -D packageName=ridb-client \
  $@

