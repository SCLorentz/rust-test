.equ SYS_WRITE, 64          // syscall número 64 (write)
.equ STDOUT, 1              // file descriptor 1 para stdout

.global write_arm_linux
.type write_arm_linux, %function

write_arm_linux:
    mov x8, SYS_WRITE       // syscall número 64 (write)
    mov x0, STDOUT          // fd = 1 (stdout)
    ldr x1, [sp]            // endereço da string (buf)
    ldr x2, [sp, #8]        // comprimento da string (count)
    svc #0                  // chamada de sistema
    ret
