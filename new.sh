#!/bin/bash

# exit when met error
set -e

# argument detect
if [[ -z "$1" ]]; then
  echo "No argument"
  exit 1
fi

# get file name and substitute the - with _
FILE_NAME=${1//-/_}

# the leetcode solution template
TEMPLATE="#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]

}

#[test]
fn test() {

}"

# create new module
echo -e "\nmod $FILE_NAME;" >> ./src/leetcode_cn/mod.rs

# create new file
echo "$TEMPLATE" > ./src/leetcode_cn/$FILE_NAME.rs

# get editor, with nvim as the default value
EDITOR=${EDITOR:-nvim}

# +6 is for neovim/vim only
$EDITOR +6 ./src/leetcode_cn/$FILE_NAME.rs
