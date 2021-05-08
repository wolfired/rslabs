; in `hello.asm`

        global _start

        section .text

_start: mov ebx, 1      ; stdout fd
        mov ecx, msg
        mov edx, 9      ; 8 chars + newline
        mov eax, 4      ; write syscall
        int 0x80

        xor ebx, ebx    ; return code 0
        mov eax, 1     ; exit syscall
        int 0x80
        
        section .data

msg:    db "hi there", 10
