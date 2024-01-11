#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use nrf51_hal::timer::Timer;
use nrf51_hal::twi;
use nrf51_hal::pac;
use nrf51_hal::gpio::p0;
use nrf51_hal::Twi;
use nrf51_hal::prelude::*;

const ACCELEROMETER_ADDR: u8 = 0x18;

const ACCELEROMETER_ID_REG: u8 = 0x00;

const ACCELEROMETER_X_REG: u8 = 0x02;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hey BMX055");
    
    let p = pac::Peripherals::take().unwrap();
    let port0 = p0::Parts::new(p.GPIO);

    let scl = port0.p0_19.into_floating_input().degrade();
    let sda = port0.p0_20.into_floating_input().degrade();

    let pins = twi::Pins { scl, sda };

    let mut i2c = Twi::new(p.TWI0, pins, twi::Frequency::K100);
            
    let mut acc_id = [0];
    let mut acc_x = [0,0];

    let mut timer = Timer::new(p.TIMER0);

    loop {

        i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc_id).unwrap();

        rprintln!("The accelerometer chip's id is: {:#b}", acc_id[0]);

        i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_X_REG], &mut acc_x).unwrap();

        rprintln!("The accelerometer chip's x is: {:#x} {:#x}", acc_x[0], acc_x[1]);

        timer.delay_ms(1000u16);

    }
}
