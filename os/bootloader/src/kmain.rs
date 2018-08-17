#![feature(asm, lang_items)]

extern crate xmodem;
extern crate pi;
use std::io::{Read,Write};

use std::fmt;
pub mod lang_items;
/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
fn jump_to(addr: *mut u8) -> ! {
    unsafe {
        asm!("br $0" : : "r"(addr as usize));
        loop { asm!("nop" :::: "volatile")  }
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    // FIXME: Implement the bootloader.
    let mut uart=pi::uart::MiniUart::new();
    let mut gpio16=pi::gpio::Gpio::new(16).into_output();
    let mut gpio18=pi::gpio::Gpio::new(18).into_output();
    
    uart.set_read_timeout(750);
    gpio16.set();
    pi::timer::spin_sleep_ms(3000);
    // fmt::Write::write_str(&mut uart,"start receiving...\n");
    let output=unsafe{std::slice::from_raw_parts_mut(BINARY_START,MAX_BINARY_SIZE)};
    loop {
        gpio18.set();
        match xmodem::Xmodem::receive(&mut uart,&mut output[..]) {
            Ok(u) =>{
                break;
            },
            Err(_) => {
            },
        }
        gpio18.clear();
        pi::timer::spin_sleep_ms(100);

    }

    gpio16.clear();
    pi::timer::spin_sleep_ms(100);
    
    // let mut buf=[0u8;100];
    // let l=uart.read(&mut buf[..]);

    // match l {
    //     Ok(u) => {
    //         uart.write(&buf[..u]);
            
    //     },
    //     Err(_) => {fmt::Write::write_str(&mut uart,"read fail\n");}
    // }
    // fmt::Write::write_str(&mut uart,"finish receiving...\n");
    
    jump_to(BINARY_START);
}
