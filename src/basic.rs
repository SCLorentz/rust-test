use crate::String;

use core::arch::asm;
use libc;

#[cfg(all(not(windows)))]
pub fn exit(code: u8) -> ! {
    unsafe { libc::exit(code as u16 as i16 as libc::c_int) }
}

#[cfg(target_arch = "x86_64")]
pub fn write<const N: usize>(text: String<N>)
{
    unsafe
    {
        let size = text.len();
        let fd = 1;                // stdout

        let syscall_ret: u64;
        asm!(
            "mov rax, 1",               // sys_write
            "mov rdi, {fd:r}",          // file descriptor
            "mov rsi, {msg}",           // endereço da mensagem
            "mov rdx, {size}",          // tamanho da mensagem
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