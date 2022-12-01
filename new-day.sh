#! /bin/zsh

cargo new day$1
cd day$1/src
rm main.rs
curl -sSLk "https://adventofcode.com/2022/day/$1/input" > input.txt
mkdir bin
cd bin
cp ../../../template.rs day$1-1.rs
