section .bss
a: resq 1
section .text
global _start:
_start:
extern debug_i64
_0:
mov qword [a], 0
_1:
mov rcx, qword [a]
mov rbx, 10
cmp rcx, rbx
mov rbx, 0
setl bl
cmp rbx, 1
je _2
jmp _3
_2:
mov rbx, qword [a]
mov rcx, 2
add rbx, rcx
mov qword [a], rbx
mov rdi, qword [a]
call debug_i64
jmp _1
_3:
mov rax, 60
mov rdi, 0
syscall
