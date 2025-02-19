#![no_std]
#![no_main]

#[macro_use]
mod functions;
use core::str;

use functions::*;
use functions::write::write;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(1)
}

fn enter_raw_mode()
{
    write(b"\x1B[2J\x1B[H");
    write(b"\x1B[?25l");
}

fn exit_raw_mode()
{
    write(b"\x1B[?25h");
}

fn sleep(ms: u64) {
    for _ in 0..(ms * 10_000)
    {
        core::hint::spin_loop();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> !
{
    enter_raw_mode();

    let msg = b"
=================================================================
*                                                               *
*          \x1b[37;43m HELLO, WORLD! \x1b[0m                                      *
*          Thanks to a woman,                                   *
*          \x1b[4mKathleen Booth\x1b[0m, this code works!                     *
*                                                               *
=================================================================\n";
    write(msg);

    // blinking_text(b"\x1b[31mHello, World!\x1b[0m");

    write(b"\n");

    sleep(15000);

    exit_raw_mode();

    exit(0)
}

/*fn blinking_text(str: &[u8])
{
    loop
    {
        write(str);
        sleep(5000);
        write(b"\r\x1b[K");
        sleep(5000);   
    }
}*/