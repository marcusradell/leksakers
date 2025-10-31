#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{Board, display::blocking::Display, hal::Timer};
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let image = [
        [0, 0, 0, 0, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 0, 0, 0, 0],
    ];

    let second_image = [
        [1, 0, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1],
    ];

    loop {
        display.show(&mut timer, image, 1000);
        display.show(&mut timer, second_image, 1000);
    }
}
