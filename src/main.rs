#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit as _;
// use panic_halt as _;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    panic!("test-panic");
    let _y;
    let x = 42;
    _y = x;
    loop {}
}
