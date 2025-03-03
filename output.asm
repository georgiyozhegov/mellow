section .bss
b: resq 1
a: resq 1
section .text
global _start
_start:
_0:
mov rbx, 5
mov [b], rbx
mov rbx, [b]
mov rcx, 2
add rbx, rcx
mov rbx, rbx
mov [a], rbx
mov rax, 60
mov rdi, [a]
syscall
