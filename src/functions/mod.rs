use core::arch::asm;

pub mod write;
pub mod file;
pub mod read;
pub mod base;

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
extern "C" { fn exit_x86_64(code: u8) -> !; }

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub fn exit(code: u8) -> ! { unsafe { exit_amdl(code) } }

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
extern "C" { fn exit_arml(code: u8) -> !; }

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub fn exit(code: u8) -> ! { unsafe { exit_arml(code) } }

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

/*fn enter_raw_mode()
{
    write(b"\x1B[2J\x1B[H");
    write(b"\x1B[?25l");
}

fn exit_raw_mode()
{
    write(b"\x1B[?25h");
}

fn sleep(ms: u64)
{
    for _ in 0..(ms * 10_000)
    {
        core::hint::spin_loop();
    }
}*/