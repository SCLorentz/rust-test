#![no_std]
#![no_main]

#[macro_use]
mod functions;
use functions::*;
use functions::write::write;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(1)
}

#[no_mangle]
pub extern "C" fn _start() -> !
{
    let msg = b"
    *****************************************************************
    *                                                               *
    *          HELLO, WORLD!                                        *
    *          Thanks to a woman,                                   *
    *          Kathleen Booth, this code works!                     *
    *                                                               *
    *****************************************************************\n";
    write(msg);

    exit(0)
}