use crate::functions::asm;
use crate::write;

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub fn read(text: &[u8]) -> [u8; 64]
{
    write(text);

    let mut input = [0u8; 64];

    // * "clobbered" significa que o registrador (x0, x1, x2, x8) será alterado
    // ? acho que de certa forma algo a ser comparado com uma variavel mut

    unsafe {
        asm!(
            "mov x0, 0",                    // 0 = stdin --> sys_read
            "mov x1, {msg}",                // Load address of the buffer
            "mov x2, {size}",               // Load size of the buffer
            "mov x8, 63",                   // Unix read system call number
            "svc 0",                        // Call kernel to read the string
            msg = in(reg) input.as_mut_ptr(),
            size = in(reg) input.len(),
            out("x0") _,                    // Mark x0 as clobbered
            out("x1") _,                    // Mark x1 as clobbered
            out("x2") _,                    // Mark x2 as clobbered
            out("x8") _,                    // Mark x8 as clobbered
            options(nostack, preserves_flags)
        );
    }
    input
}