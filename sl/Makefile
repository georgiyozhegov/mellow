compile:
	mkdir -p output
	gcc -nostdlib -no-pie -fno-stack-protector -c debug.c -o output/debug.o
	ld -r output/debug.o -o output/main.o

clean:
	rm output/*
