.global exit_armx
exit_armx:
    mov x16, #1
    svc 0x80
