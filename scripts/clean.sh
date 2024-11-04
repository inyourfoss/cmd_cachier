#!/usr/bin/env sh   

# Remove all files that have a corresponding templ file
find . -type f -name "*.templ" -exec sh -c 'rm -f ${0%.templ}' {} \;
find . -type d -name "target" -exec rm -rf {} \;
find . -type f -name "Cargo.lock" -exec rm -f {} \;
