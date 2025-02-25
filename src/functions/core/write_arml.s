.global write_arml
write_arml:
    mov x0, 1
    mov x8, 64
    svc 0x80
    ret
