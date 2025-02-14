#![no_std]
#![no_main]

use core::arch::asm;
use exit_no_std::exit;

mod string;
use string::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(1)
}

#[no_mangle]
pub extern "C" fn _start()
{
    unsafe
    {
        let msg: String<12> = String::new(b"Hello, World");
        let size = msg.len();
        let fd = 1;            // stdout

        let syscall_ret: u64;
        asm!(
            "mov rax, 1",           // sys_write
            "mov rdi, {fd:r}",      // file descriptor
            "mov rsi, {msg}",       // endereço da mensagem
            "mov rdx, {size}",      // tamanho da mensagem
            "syscall",
            fd = in(reg) fd,
            msg = in(reg) msg.as_bytes().as_ptr(),
            size = in(reg) size,
            out("rax") syscall_ret,
            options(nostack, preserves_flags)
        );

        if syscall_ret != size as u64 {
            exit(1)
        }

        exit(0)
    }
}