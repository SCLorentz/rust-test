#![no_std]
#![no_main]

mod functions;
use functions::*;

mod string;
use string::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(1)
}

#[no_mangle]
pub extern "C" fn _start()
{
    let msg: String<13> = String::new(b"Hello, World!");
    write(msg);

    exit(0)
}