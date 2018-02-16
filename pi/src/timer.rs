use common::IO_BASE;
use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

/// The base address for the ARM system timer registers.
const TIMER_REG_BASE: usize = IO_BASE + 0x3000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    CS: Volatile<u32>,
    CLO: ReadVolatile<u32>,
    CHI: ReadVolatile<u32>,
    COMPARE: [Volatile<u32>; 4]
}

/// The Raspberry Pi ARM system timer.
pub struct Timer {
    registers: &'static mut Registers
}

impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut Registers) },
        }
    }

    /// Reads the system timer's counter and returns the 64-bit counter value.
    /// The returned value is the number of elapsed microseconds.
    pub fn read(&self) -> u64 {
        loop {
            let hb_old:u64=u64::from(self.registers.CHI.read());
            let lb:u64=u64::from(self.registers.CLO.read());
            let hb:u64=u64::from(self.registers.CHI.read());
            if hb_old==hb{
                let res:u64=(hb<<32)+lb;
                return res;
            }
        }
    }
}

/// Returns the current time in microseconds.
pub fn current_time() -> u64 {
    let val=Timer::new().read();
    val
}

/// Spins until `us` microseconds have passed.
pub fn spin_sleep_us(us: u64) {
    let cur=current_time();
    loop{
        if cur+us<=current_time(){
            break
        }
    }
}

/// Spins until `ms` milliseconds have passed.
pub fn spin_sleep_ms(ms: u64) {
    spin_sleep_us(ms*1000);
}
