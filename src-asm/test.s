.global _start

.section .text
_start:
    mov x8, 64           // syscall write
    mov x0, 1            // STDOUT
    ldr x1, =message     // mensagem
    mov x2, 13           // tamanho da mensagem
    svc #0               // chamada de sistema
    mov x8, 93           // syscall exit
    mov x0, 0            // código de saída
    svc #0               // chamada de sistema

.section .data
message:
    .asciz "Hello, world!"