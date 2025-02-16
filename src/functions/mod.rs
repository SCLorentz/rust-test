use core::arch::asm;

pub mod write;
pub mod file;

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[no_mangle]
pub extern "C" fn exit(code: u8) -> !
{
    unsafe {
        asm!(
            "mov rax, 60",                      // sys_exit
            "mov dil, {code}",                  // dil é o registrador de 8 bits que corresponde ao registrador rdi
            "syscall",
            code = in(reg_byte) code,
            options(noreturn)
        );
    }
}

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[no_mangle]
pub extern "C" fn exit(code: u8) -> !
{
    unsafe {
        asm!(
            "mov x0, {code:x}",
            "mov x8, 93",                       // sys_exit
            "svc #0",
            code = in(reg) code,
            options(noreturn)
        );
    }
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
#[no_mangle]
pub extern "C" fn exit(code: u8) -> !
{
    unsafe {
        asm!(
            "mov x0, {code:x}",
            "mov x16, #1",                      // sys_exit
            "svc 0x80",
            code = in(reg) code,
            options(noreturn)
        );
    }
}