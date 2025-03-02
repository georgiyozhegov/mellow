#!/bin/fish
cargo run > output.asm
cat output.asm
nasm -g -f elf64 output.asm -o output.o
ld -g output.o -o output
rm output.o
./output
echo $status
