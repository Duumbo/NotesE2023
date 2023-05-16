SECTION .text
    GLOBAL _start

    _start:
        ; print name
        mov rax,1     ; sys_write
        mov rdi,1     ; stdout
        mov rsi,5
        mov rcx,5
        shl rsi,2
        and rsi,rcx
        mov rdx,8 ; length
        syscall

        ; exit program
        mov rax,60    ; sys_exit
        mov rdi,0     ; success
        syscall

