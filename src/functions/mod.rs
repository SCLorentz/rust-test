use crate::String;

use core::arch::asm;
use libc;

#[cfg(all(not(windows)))]
pub fn exit(code: u8) -> ! {
    unsafe { libc::exit(code as u16 as i16 as libc::c_int) }
}

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub fn write<const N: usize>(text: String<N>)
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
            "svc 0",                            // Call kernel to write the string
            out("x0") _,
            text_ptr = in(reg) text_ptr,
            text_len = in(reg) text_len,
            options(nostack, preserves_flags),
        )
    }
}

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub fn write<const N: usize>(text: String<N>)
{
    unsafe
    {
        let size = text.len() as usize; // ? idk if this helps but the other version (arm) worked
        let fd = 1;                     // stdout

        let syscall_ret: u64;
        asm!(
            "mov rax, 1",                   // sys_write
            "mov rdi, {fd:r}",              // file descriptor
            "mov rsi, {msg}",               // Load address of the string
            "mov rdx, {size}",              // Load length of the string
            "syscall",
            fd = in(reg) fd,
            msg = in(reg) text.as_bytes().as_ptr(),
            size = in(reg) size,
            out("rax") syscall_ret,
            options(nostack, preserves_flags)
        );

        if syscall_ret != size as u64 { return }
    }
}