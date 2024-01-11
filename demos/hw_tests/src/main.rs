#![no_std]
#![no_main]

use panic_rtt_target as _;
//use panic_halt as _;
use rtt_target::{rtt_init_print, rprintln};

use cortex_m_rt::entry;

use calliope::Board;

use calliope::hal::timer::Timer;
use calliope::hal::timer::Instance;
use calliope::hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use calliope::hal::prelude::OutputPin;
use calliope::{pin4_set_high_outset, pin4_set_high_gpio, pin4_set_high_asm};
use calliope::hal::gpio::PushPull;
use calliope::hal::gpio::Output;

type Cpin<'a> = &'a mut calliope::hal::gpio::Pin<Output<PushPull>>;

fn led_test<T :Instance, U>( timer : &mut Timer<T,U>, col1: Cpin){
        let delay = 500u32;
        
        rprintln!("Hello set_high");
        col1.set_high().unwrap();
        timer.delay_ms(delay);
        col1.set_low().unwrap();
        timer.delay_ms(delay);
        
        rprintln!("Hello outset");
        pin4_set_high_outset();
        timer.delay_ms(delay);
        col1.set_low().unwrap();
        timer.delay_ms(delay);

        rprintln!("Hello gpio");
        pin4_set_high_gpio();
        timer.delay_ms(delay);
        col1.set_low().unwrap();
        timer.delay_ms(delay);
        
        rprintln!("Hello asm");
        pin4_set_high_asm();
        timer.delay_ms(delay);
        col1.set_low().unwrap();
        timer.delay_ms(delay);
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut row1 = board.display_pins.row1;

    row1.set_high().unwrap();

    let col1 = board.display_pins.col1;
    let mut col1d = col1.degrade();


    loop {
        led_test(&mut timer, &mut col1d);
    }
}
