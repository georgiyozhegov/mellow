section .bss
a: resq 1
b: resq 1
section .text
global _start
_start:
_0:
mov rbx, 1
mov [a], rbx
mov rbx, 0
mov rcx, 1
sub rbx, rcx
mov rbx, rbx
mov [b], rbx
jmp _1
_1:
mov rbx, [a]
mov rcx, 100
cmp rbx, rcx
mov rbx, 0
setl bl
cmp rbx, 1
je _2
jmp _5
_2:
mov rcx, [a]
mov rbx, 1
add rcx, rbx
mov rcx, rcx
mov rbx, 2
cmp rcx, rbx
mov rbx, 0
sete bl
cmp rbx, 1
je _3
jmp _4
_3:
mov rbx, 10
mov [b], rbx
mov rcx, [a]
mov rbx, 1
add rcx, rbx
mov rcx, rcx
mov rbx, 2
cmp rcx, rbx
mov rbx, 0
sete bl
cmp rbx, 1
je _4
jmp _4
_4:
mov rcx, [a]
mov rbx, 1
add rcx, rbx
mov rbx, rcx
mov [a], rbx
jmp _1
_5:
mov rbx, [b]
mov [a], rbx
_6:
mov rax, 60
mov rdi, [a]
syscall
