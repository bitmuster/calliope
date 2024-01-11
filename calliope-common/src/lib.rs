
#![no_std]
#![allow(non_camel_case_types)]

use nrf51_hal::Timer;
use crate::hal::timer::Instance;
use nrf51_hal::prelude::_embedded_hal_blocking_delay_DelayUs;

use core::arch::asm;

pub use nrf51_hal as hal;

pub use hal::pac as pac;
// pub use hal::pac::Peripherals;


pub mod board;
pub mod gpio;

pub use board::Board;

pub use crate::board::*;

// Experimental unsafe beep function that bypasses not yet existing
// hal functionality.
// ToDo: Improve a lot
pub fn beep<T :Instance, U>(timer: &mut Timer<T,U>) {

    let pin_cnf_28 = 0x50000000usize + 0x770usize;
    let ptr_pin_cnf_28 = pin_cnf_28 as *mut u32;

    let pin_cnf_29 = 0x50000000usize + 0x774usize;
    let ptr_pin_cnf_29 = pin_cnf_29 as *mut u32;

    let pin_cnf_30 = 0x50000000usize + 0x778usize;
    let ptr_pin_cnf_30 = pin_cnf_30 as *mut u32;

    let outset = 0x50000000usize + 0x508usize;
    let ptr_outset = outset as *mut u32;

    let outclr = 0x50000000usize + 0x50Cusize;
    let ptr_outclr = outclr as *mut u32;

    let dirset = 0x50000000usize + 0x518usize;
    let ptr_dirset = dirset as *mut u32;

    unsafe { core::ptr::write_volatile(ptr_dirset, 1u32 << 28) }
    unsafe { core::ptr::write_volatile(ptr_dirset, 1u32 << 29) }
    unsafe { core::ptr::write_volatile(ptr_dirset, 1u32 << 30) }

    unsafe { core::ptr::write_volatile(ptr_pin_cnf_28, 1u32 << 0) }
    unsafe { core::ptr::write_volatile(ptr_pin_cnf_29, 1u32 << 0) }
    unsafe { core::ptr::write_volatile(ptr_pin_cnf_30, 1u32 << 0) }

    // set sleep of DRV8837 to 1
    unsafe { core::ptr::write_volatile(ptr_outset, 1u32 << 28) }

    let delay : u32 = 300;
    for _i in 1..100 {
            unsafe { core::ptr::write_volatile(ptr_outset, 1u32 << 30) }
            unsafe { core::ptr::write_volatile(ptr_outclr, 1u32 << 29) }
            timer.delay_us(delay);
            unsafe { core::ptr::write_volatile(ptr_outclr, 1u32 << 30) }
            unsafe { core::ptr::write_volatile(ptr_outset, 1u32 << 29) }
            timer.delay_us(delay);
    }
}

// Set a color on the WS2812
// Needs inline assembler due to critical timing
// ToDo: Bug: Seems to set the wrong color directly after poweron?
// ToDo: Improve and check timing
// ToDo: Try the same with SPI
// ToDo: Try to improve to run N Leds on a GPIO
pub fn set_ws2812( r:u8, g: u8, b:u8) {

    let val = ((g as u32) << 24) + ((r as u32) << 16) + ((b as u32) << 8);

    //rprintln!("Value {:08x}", val);

    let pin_cnf_18 = 0x50000000usize + 0x748usize;
    let ptr_pin_cnf_18 = pin_cnf_18 as *mut u32;

    //let pin_cnf_22 = 0x50000000usize + 0x758usize;
    //let ptr_pin_cnf_22 = pin_cnf_22 as *mut u32;

    let outset : u32 = 0x50000000 + 0x508;

    let outclr : u32 = 0x50000000 + 0x50C;

    // Enable pin 18 as output
    unsafe { core::ptr::write_volatile(ptr_pin_cnf_18, 1u32 << 0) }
    let pin :u32 = 1u32 << 18;
    //To test: switch to test port 3 -> pin 22
    //unsafe { core::ptr::write_volatile(ptr_pin_cnf_22, 1u32 << 0) }
    //let pin :u32 = 1u32 << 22;

    //let mut index: u32 = 0; Avoid compiler warning
    let index: u32 = 0;

    unsafe {

        //val = val +1;

        asm!(
            "1:", // preamble

            "ADDS {1}, {1}, #1",
            "CMP {1}, #24",
            "BHI 2f",

            "LSLS {0}, {0}, #1",
            "BCS 4f",

            //send zero

            // set pin
            "STR  {2}, [{3}]",

            // wait 0.4 us
            "nop",
            "nop",
            "nop",
            "nop",

            // clear pin
            "STR  {2}, [{4}]",

            // wait 0.8 us
            "nop",
            "nop",
            "nop",
            "nop",
            // Four nop removed from calculated wait time due to the
            // preamble with some instructions.
            //"nop",
            //"nop",
            //"nop",
            //"nop",

            "B 1b",

            //send one
            "4:",
            // set pin
            "STR  {2}, [{3}]",

            // wait 0.8 us
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",

            // clear pin
            "STR  {2}, [{4}]",

            // wait 0.4 us
            // Four nop removed from calculated wait time due to the
            // preamble with some instructions.
            //"nop",
            //"nop",
            //"nop",
            //"nop",

            "B 1b",

            "2:", // End label

            in(reg) val,
            in(reg) index,
            in(reg) pin,
            in(reg) outset,
            in(reg) outclr,
        );
    }
}

//Ways to access ports(examples)

pub fn pin4_set_high_outset() {
    let outset = 0x50000000usize + 0x508usize;
    let ptr_outset = outset as *mut u32;
    unsafe { core::ptr::write_volatile(ptr_outset, 1u32 << 4) }
}

pub fn pin4_set_high_gpio() {
    unsafe { (*pac::GPIO::ptr()).outset.write(|w| w.bits(1u32 << 4)); }
}

pub fn pin4_set_high_asm() {
    let outset : u32 = 0x50000000 + 0x508;
    let pin :u32 = 1u32 << 4;
    unsafe{
        asm!(
            "STR  {1}, [{0}]",
            in(reg) outset,
            in(reg) pin,
        );
    }
}



