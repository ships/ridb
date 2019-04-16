#!/bin/bash -eux

this_dir="$(cd "$(dirname $0)" && pwd )"

clean() {
  which yq
    rm -rf src docs Cargo.* ridb-formatted.yaml
    yq r ridb.yaml > ridb-formatted.yaml
}

generate() {
  openapi-generator generate \
    --skip-validate-spec \
    -i "$this_dir/ridb-formatted.yaml" \
    -o "$this_dir" \
    -g rust \
    -D packageName=ridb-client \
    --library=reqwest \
    $@
}

fix_queryname() {
  sed -i '' '/let query_string/,/let uri_str/s/ query/ query_builder/g' src/apis/organizations_api.rs
}

regress_gitignore() {
  git co .gitignore
}

pushd "$this_dir"
  clean
  generate
  fix_queryname
  regress_gitignore
popd
