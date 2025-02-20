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
=================================================================

\"Like a girl\" - Como uma garota
---
A frase como uma garota, as vezes, especialmente no cotidiano de
muitas mulheres, pode ser algo comum de se ouvir. Mas o que isso
significa?

Bem, para muitas crian\xC3\xA7as, isso parecia algo dito com inten\xC3\xA7\xC3\xB5es
ruins. Para muitas mulheres j\xC3\xA1 adultas isso representava um
trauma, pois \xC3\xA9 algo muitas vezes usado como justificativa sobre
o que uma mulher pode e n\xC3\xA3o pode fazer, como se ela n\xC3\xA3o tivesse
capacidade de fazer tal coisa.

Algo relatado no video \xC3\xA9 como durante o inicio da puberdade a
confian\xC3\xA7a dessas garotas diminui, e especialmente nessa fase, os
comentarios realmente podem machucar. E as intrevistadas
afirmaram que fariam as coisas de forma diferente caso ouvissem
isso hoje. Correriam do seu jeito e continuariam com o que
estavam fazendo. Porque elas correm como garotas, escovam os
dentes como garotas e acordam como garotas, porque elas s\xC3\xA3o
garotas.\n";

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