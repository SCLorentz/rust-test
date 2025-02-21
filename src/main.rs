#![no_std]
#![no_main]

#[macro_use]
mod functions;
use functions::{exit, write::write};

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
use functions::write::read;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { exit(1) }

#[no_mangle]
pub extern "C" fn _start() -> !
{
    write(b"Hello, World!\n");

    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    format!(b"your input was: ", &read(b"input: "));

    exit(0)
}