section .bss
a: resq 1
section .text
global _start:
_start:
extern debug_i64
_0:
mov qword [a], 20
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
mov rdi, 0
call debug_i64
jmp _8
_3:
mov rcx, qword [a]
mov rbx, 10
cmp rcx, rbx
mov rbx, 0
sete bl
cmp rbx, 1
je _4
jmp _5
_4:
mov rdi, 1
call debug_i64
jmp _8
_5:
mov rcx, qword [a]
mov rbx, 5
cmp rcx, rbx
mov rbx, 0
sete bl
cmp rbx, 1
je _6
jmp _7
_6:
mov rdi, 2
call debug_i64
jmp _8
_7:
mov rdi, 3
call debug_i64
_8:
_9:
mov rax, 60
mov rdi, 0
syscall
