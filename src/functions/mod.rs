use core::arch::asm;

pub mod write;
pub mod read;
pub mod base;

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
extern "C" { fn exit_amdl(code: u8) -> !; }

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub fn exit(code: u8) -> ! { unsafe { exit_amdl(code) } }

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
extern "C" { fn exit_arml(code: u8) -> !; }

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub fn exit(code: u8) -> ! { unsafe { exit_arml(code) } }

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
extern "C" { fn exit_armx(code: u8) -> !; }

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
pub fn exit(code: u8) -> ! { unsafe { exit_armx(code) } }

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