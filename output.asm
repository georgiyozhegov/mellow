section .bss
a: resq 1
section .text
global _start
_start:
_0:
mov rbx, 0
mov [a], rbx
mov rbx, 0
cmp rbx, 1
je _1
jmp _2
_1:
mov rbx, 1
mov [a], rbx
mov rbx, 1
cmp rbx, 1
je _2
jmp _3
_2:
mov rbx, 2
mov [a], rbx
mov rbx, 1
cmp rbx, 1
je _4
jmp _3
_3:
mov rbx, 3
mov [a], rbx
_4:
mov rax, 60
mov rdi, [a]
syscall
