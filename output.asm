section .bss
a: resq 1
section .text
global _start
_start:
_0:
mov rcx, 4
mov rbx, 2
mov rax, rcx
cqo
idiv rbx
mov rbx, rax
mov rcx, 1
mov rsi, 2
imul rcx, rsi
mov rcx, rcx
add rbx, rcx
mov rbx, rbx
mov [a], rbx
mov rax, 60
mov rdi, [a]
syscall
