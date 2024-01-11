#![no_std]
#![no_main]

// use panic_rtt_target as _;
use panic_halt as _;

use cortex_m_rt::entry;

use calliope::Board;

use calliope::hal::timer::Timer;
use calliope::hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use calliope::hal::prelude::OutputPin;


#[entry]
fn main() -> ! {

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let delay = 1000u32;
    let mut col1 = board.display_pins.col1;
    let mut row1 = board.display_pins.row1;

    col1.set_low().unwrap();

    loop {
        row1.set_high().unwrap();
        timer.delay_ms(delay);
        row1.set_low().unwrap();
        timer.delay_ms(delay);
    }
}
