compile:
	mkdir -p output
	cargo r > output/main.asm
	make -C sl compile
	nasm -f elf64 output/main.asm -o output/main.o
	gcc -nostdlib -no-pie output/main.o sl/output/main.o -o output/main

clean:
	make -C sl clean
	rm output/*

run: compile
	./output/main
