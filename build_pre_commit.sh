#!/usr/bin/env bash

# build_pre_commit.sh - build pre-commit binary

cd `dirname $0`

printf "\nBuilding ...\n\n"

cargo build --release && echo
