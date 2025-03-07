section .bss
a: resq 1
section .text
global _start:
_start:
extern debug_i64
_0:
mov rbx, 1
mov [a], rbx
_1:
mov rcx, [a]
mov rbx, 10
cmp rcx, rbx
mov rbx, 0
setl bl
cmp rbx, 1
je _2
jmp _3
_2:
mov rbx, [a]
mov rdi, rbx
call debug_i64
mov rcx, [a]
mov rbx, 1
add rcx, rbx
mov rbx, rcx
mov [a], rbx
jmp _1
_3:
mov rax, 60
mov rdi, 0
syscall
