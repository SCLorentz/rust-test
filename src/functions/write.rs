use crate::String;
use core::arch::asm;

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub fn write(text: String)
{
    unsafe
    {
        let size = text.len() as usize;

        let syscall_ret: u64;
        asm!(
            "mov rax, 1",                   // sys_write
            "mov rdi, 1",                   // file descriptor
            "mov rsi, {msg}",               // Load address of the string
            "mov rdx, {size}",              // Load length of the string
            "syscall",
            msg = in(reg) text.as_bytes().as_ptr(),
            size = in(reg) size,
            out("rax") syscall_ret,
            options(nostack, preserves_flags)
        );

        if syscall_ret != size as u64 { return }
    }
}

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub fn write(text: String)
{
    unsafe
    {
        let text_ptr = text.as_bytes().as_ptr();
        let text_len = text.len() as usize;

        asm!(
            "mov x0, 1",                        // 1 = StdOut --> sys_write
            "mov x1, {text_ptr}",               // Load address of the string
            "mov x2, {text_len}",               // Load length of the string
            "mov x8, 64",                       // Unix write system call
            "svc 0x80",                            // Call kernel to write the string
            out("x0") _,
            text_ptr = in(reg) text_ptr,
            text_len = in(reg) text_len,
            options(nostack, preserves_flags),
        )
    }
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
pub fn write(text: String)
{
    unsafe
    {
        let text_ptr = text.as_bytes().as_ptr();
        let text_len = text.len() as usize;

        asm!(
            "mov x0, 1",                        // 1 = StdOut --> sys_write
            "mov x1, {text_ptr}",               // Load address of the string
            "mov x2, {text_len}",               // Load length of the string
            "mov x16, #4",                       // Unix write system call
            "svc 0",                            // Call kernel to write the string
            out("x0") _,
            text_ptr = in(reg) text_ptr,
            text_len = in(reg) text_len,
            options(nostack, preserves_flags),
        )
    }
}

/*
.macro write str
    mov     X0, SYS_WRITE              	// 1 = StdOut --> sys_write
    adr     X1, \str                    // Load address of the string
    ldr     X2, =(\str\()_end - \str)   // Calculate length of the string
    mov     X16, #4                     // Unix write system call
    svc     KERNEL                      // Call kernel to write the string
.endm
*/