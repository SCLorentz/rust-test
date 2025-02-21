use crate::functions::asm;

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub fn write(text: &[u8])
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
            msg = in(reg) text.as_ptr(),
            size = in(reg) size,
            out("rax") syscall_ret,
            options(nostack, preserves_flags)
        );

        if syscall_ret != size as u64 { return }
    }
}

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub fn write(text: &[u8])
{
    unsafe
    {
        let text_ptr = text.as_ptr();
        let text_len = text.len() as usize;

        asm!(
            "mov x0, 1",                        // 1 = StdOut --> sys_write
            "mov x1, {text_ptr}",               // Load address of the string
            "mov x2, {text_len}",               // Load length of the string
            "mov x8, 64",                       // Unix write system call
            "svc 0x80",                         // Call kernel to write the string
            out("x0") _,
            text_ptr = in(reg) text_ptr,
            text_len = in(reg) text_len,
            options(nostack, preserves_flags),
        )
    }
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
pub fn write(text: &[u8])
{
    unsafe
    {
        let text_ptr = text.as_ptr();
        let text_len = text.len() as usize;

        asm!(
            "mov x0, 1",                        // 1 = StdOut --> sys_write
            "mov x1, {text_ptr}",               // Load address of the string
            "mov x2, {text_len}",               // Load length of the string
            "mov x16, #4",                      // Unix write system call
            "svc 0x80",                         // Call kernel to write the string
            out("x0") _,
            text_ptr = in(reg) text_ptr,
            text_len = in(reg) text_len,
            options(nostack, preserves_flags),
        )
    }
}

#[macro_export]
macro_rules! format
{
    ($($arg:expr),*) => {{
        let mut buffer = [0u8; 256]; // Buffer temporário (ajuste conforme necessário)
        let mut index = 0;

        $(
            let bytes = $arg;
            if index + bytes.len() < buffer.len() {
                buffer[index..index + bytes.len()].copy_from_slice(bytes);
                index += bytes.len();
            }
        )*

        write(&buffer[..index]);
    }};
}