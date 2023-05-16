;
; assemble and link with:
; nasm -f elf printf-test.asm && gcc -m32 -o printf-test printf-test.o
;
section .text
global main
extern printf

main:

  mov eax, 0xDEADBEEF
  push eax
  push message
  call printf
  add esp, 8
  ret

message db "Register = %08X", 10, 0
