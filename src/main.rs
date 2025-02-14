#![no_std]
#![no_main]

use core::arch::asm;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    // Código para imprimir a mensagem
    unsafe {
        let msg = b"Hello, World!";
        let size = msg.len();
        let fd = 1; // stdout

        let syscall_ret: u64;
        asm!(
            "mov rax, 1",      // sys_write
            "mov rdi, {}",     // file descriptor
            "mov rsi, {}",     // endereço da mensagem
            "mov rdx, {}",     // tamanho da mensagem
            "syscall",
            in(reg) fd,
            in(reg) msg.as_ptr(),
            in(reg) size,
            out("rax") syscall_ret,
            options(nostack, preserves_flags)
        );

        if syscall_ret != size as u64 {
            loop {}
        }

        loop {}
    }
}