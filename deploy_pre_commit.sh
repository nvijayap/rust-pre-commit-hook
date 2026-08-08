#!/usr/bin/env bash

# deploy_pre_commit.sh - copies pre-commit to a local git repo's root dir

if [ $# -ne 1 ]; then
  printf "\nNeed local git repo's path as arg\n\n"; exit 1
fi

cp target/release/pre-commit $1/.git/hooks/
