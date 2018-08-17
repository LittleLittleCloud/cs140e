#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(pointer_methods)]

extern crate pi;
extern crate stack_vec;
use std::fmt;
use std::io::Read;
use std::io::Write;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;
#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let mut gpio16=pi::gpio::Gpio::new(16).into_output();
	let mut gpio18=pi::gpio::Gpio::new(18).into_output();
	// let mut gpio26=pi::gpio::Gpio::new(26).into_output();
	gpio16.set();
	pi::timer::spin_sleep_ms(3000);
	// uart.set_read_timeout(1000);
	shell::shell(">");
}
