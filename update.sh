#!/usr/bin/env bash

set -xe

# INSTALL DEPENDENCIES

cargo install --force --version 0.17.0 svd2rust
cargo install --force --version 0.7.0 form

# PATCH SVD FILES AND GENERATE CRATES

TOP="${PWD}"

xsl=svd/devices/atsam4e16e.xsl
chip=$(basename "${xsl}" .xsl)
CHIP=$(echo "${chip}" | tr '[:lower:]' '[:upper:]')
svd=svd/${CHIP}.svd

xsltproc "${TOP}/${xsl}" "${TOP}/${svd}" | svd2rust

rm -rf src/
form -i lib.rs -o src
rm lib.rs
cargo fmt
rustfmt build.rs
