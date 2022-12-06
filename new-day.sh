#! /bin/zsh

curl -sSLk "https://adventofcode.com/2022/day/$1/input" > src/input/day$1-input.txt
cp template.rs src/bin/day$1-1.rs
