section .bss
a: resq 1
b: resq 1
section .text
global _start
_start:
_0:
mov rbx, 1
mov rcx, 3
add rbx, rcx
mov rcx, rbx
mov rbx, 3
add rcx, rbx
mov rbx, rcx
mov [a], rbx
mov rbx, 2
mov rcx, 3
add rbx, rcx
mov rbx, rbx
mov rcx, 2
add rbx, rcx
mov rbx, rbx
mov [b], rbx
mov rax, 60
mov rdi, [a]
syscall
