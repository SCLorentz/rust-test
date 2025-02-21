use core::arch::asm;

pub mod write;
pub mod read;
pub mod base;

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[link(name = "exit_amdl")]
extern "C" { fn exit_amdl(code: u8) -> !; }

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub fn exit(code: u8) -> ! { unsafe { exit_amdl(code) } }

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[link(name = "exit_arml")]
extern "C" { fn exit_arml(code: u8) -> !; }

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub fn exit(code: u8) -> ! { unsafe { exit_arml(code) } }

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
#[link(name = "exit_armx")]
extern "C" { fn exit_armx(code: u8) -> !; }

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
pub fn exit(code: u8) -> ! { unsafe { exit_armx(code) } }