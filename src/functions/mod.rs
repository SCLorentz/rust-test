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
            "mov x0, 1",                    // file descriptor (1 para stdout)
            "mov x1, {text_ptr}",           // endereço da string
            "mov x2, {text_len}",           // tamanho da string
            "mov x8, 64",                   // número da chamada de sistema write
            "svc 0",                        // chamada de sistema
            out("x0") _,                    // resultado da chamada de sistema
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
        let size = text.len();
        let fd = 1;                     // stdout

        let syscall_ret: u64;
        asm!(
            "mov rax, 1",                   // sys_write
            "mov rdi, {fd:r}",              // file descriptor
            "mov rsi, {msg}",               // endereço da mensagem
            "mov rdx, {size}",              // tamanho da mensagem
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