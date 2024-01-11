#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use cortex_m_rt::entry;

use calliope::Board;
use calliope::beep;

use calliope::hal::timer::Timer;
//use calliope::hal::pac::TIMER0;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World");

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    rprintln!("Here comes the beep");
    beep(&mut timer);

    loop {}
}
