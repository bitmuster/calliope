#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

use cortex_m_rt::entry;

use calliope::Board;
use calliope::set_ws2812;
use calliope::hal::timer::Timer;
use calliope::hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use calliope::hal::timer::Instance;

enum LedState {
    GreenToYellow,
    YellowToRed,
    RedToPink,
    PinkToBlue,
    BlueToCyan,
    CyanToGreen,
}


fn animation <T :Instance, U>( timer : &mut Timer<T,U>) -> ! {

    let max = 0x20;
    let min = 0x00;
    let d = 50u16;
    let mut state: LedState = LedState::GreenToYellow;

    let mut g:u8 = max;
    let mut r:u8 = 0;
    let mut b:u8 = 0;

    loop{

    match state {
        LedState::GreenToYellow =>
            {
                r=r+1;
                if r >= max {
                    state=LedState::YellowToRed;
                }
            },
        LedState::YellowToRed =>
            {
                g=g-1;
                if g <= min {
                    state=LedState::RedToPink;
                }
            },
        LedState::RedToPink =>
            {
                b=b+1;
                if b >= max {
                    state=LedState::PinkToBlue;
                }
            },
        LedState::PinkToBlue =>
            {
                r=r-1;
                if r <= min {
                    state=LedState::BlueToCyan;
                }
            },
        LedState::BlueToCyan =>
            {
                g=g+1;
                if g >= max {
                    state=LedState::CyanToGreen;
                }
            },
        LedState::CyanToGreen =>
            {
                b=b-1;
                if b <= min {
                    state=LedState::GreenToYellow;
                }
            },
        }
    set_ws2812(r,g,b);
    timer.delay_ms(d);
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    rprintln!("Hello RGB LED");

    let delay = 1000u32;

    let mut r:u8;
    let mut g:u8;
    let mut b:u8;
    
    r=0x20; g=0x00; b=0x00;
    set_ws2812(r,g,b);
    timer.delay_ms(delay);

    r=0x00; g=0x20; b=0x00;
    set_ws2812(r,g,b);
    timer.delay_ms(delay);

    r=0x00; g=0x00; b=0x20;
    set_ws2812(r,g,b);
    timer.delay_ms(delay);

    r=0x00; g=0x00; b=0x00;
    set_ws2812(r,g,b);
    timer.delay_ms(delay);

    animation(&mut timer);
}
