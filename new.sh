#!/bin/bash

set -ex

if [[ -z "$1" ]]; then
  echo "No argument"
  exit 1
fi

FILE_NAME=${1//-/_}
TEMPLATE="#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]

}

#[test]
fn test() {

}"

echo "$TEMPLATE" > ./src/leetcode_cn/$FILE_NAME.rs
echo "mod $FILE_NAME;" >> ./src/leetcode_cn/mod.rs

EDITOR=${EDITOR:-nvim}

$EDITOR +6 ./src/leetcode_cn/$FILE_NAME.rs
