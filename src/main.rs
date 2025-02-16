#![no_std]
#![no_main]

#[macro_use]
mod functions;
use functions::*;
use functions::write::write;

mod types;
use types::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(1)
}

#[no_mangle]
pub extern "C" fn _start() -> !
{
    let msg: String = String::new(b"
    ****************************************
    *                                      *
    *          HELLO, WORLD!               *
    *          Bem-vindo ao Rust!          *
    *                                      *
    ****************************************\n");
    write(msg);

    exit(0)
}