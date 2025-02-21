.global exit_amdl
exit_amdl:
    movq %rdi, %rax
    movq $60, %rax
    syscall
