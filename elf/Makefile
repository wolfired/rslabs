all:
	gcc -o ./c/entry_point ./c/entry_point.c
	nasm -f elf32 ./c/hello_x32.asm -o ./c/hello_x32.o && ld -m elf_i386 ./c/hello_x32.o -o ./c/hello_x32
	nasm -f elf64 ./c/hello_x64.asm -o ./c/hello_x64.o && ld -m elf_x86_64 ./c/hello_x64.o -o ./c/hello_x64

clean:
	rm -rf ./c/entry_point
	rm -rf ./c/entry_point.o
	rm -rf ./c/hello_x32
	rm -rf ./c/hello_x32.o
	rm -rf ./c/hello_x64
	rm -rf ./c/hello_x64.o
