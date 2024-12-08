section .data
      true: dq 1
      false: dq 0
section .text
      global _start
_start:
_1:
      ; let a = 1 + 3
      mov rax, 1
      mov rbx, 3
      add rax, rbx
_2:
      ; if
      ; a > 3
      mov rbx, 3
      cmp rax, rbx
      mov rbx, 0
      cmovg rbx, [true]
      jz _2.else
      ; then
_2.then:
      mov rax, 60
      mov rdi, 1
      syscall
      jmp _2.end
      ; else
_2.else:
      mov rax, 60
      mov rdi, 2
      syscall
      ; end
_2.end:

; let a = 1 + 2
; do if a > 3 then
;     # exit 1
; else
;     # exit 2
; end
