
#![no_std]
#![allow(non_camel_case_types)]

use nrf51_hal::Timer;
use crate::hal::timer::Instance;
use nrf51_hal::prelude::_embedded_hal_blocking_delay_DelayUs;

pub use nrf51_hal as hal;

pub use hal::pac;
// pub use hal::pac::Peripherals;


pub mod board;
pub mod gpio;

pub use board::Board;

pub use crate::board::*;

// Experimental unsafe beep function that bypasses not yet existing
// hal functionality.
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

