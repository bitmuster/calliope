#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

use cortex_m_rt::entry;

use calliope::set_ws2812;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World");

    let mut r:u8 = 0x20;
    let mut g:u8 = 0x00;
    let mut b:u8 = 0x20;
    
    set_ws2812(r,g,b);

    loop {}
}
