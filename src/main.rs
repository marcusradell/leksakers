#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        Saadc, Timer,
        gpio::{Level, OpenDrainConfig},
        saadc::SaadcConfig,
    },
};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // initialize adc
    let saadc_config = SaadcConfig::default();
    let mut saadc = Saadc::new(board.ADC, saadc_config);
    let mut mic_in = board.microphone_pins.mic_in.into_floating_input();

    // enable microphone
    board
        .microphone_pins
        .mic_run
        .into_open_drain_output(OpenDrainConfig::Disconnect0HighDrive1, Level::High);

    let mut count: u64 = 0;
    let mut sum: u64 = 0;
    let mut max_value: u16 = 0;
    loop {
        let mic_value = saadc
            .read_channel(&mut mic_in)
            .expect("could not read value of microphone") as u16;

        // Smoothen the signal as audio comes in waves
        max_value = max_value.max(mic_value);
        sum += mic_value as u64;
        count += 1;

        if count % 100 == 0 {
            let avg = (sum / count) as u16;
            let image = [
                [if max_value > avg + 100 / 2 { 1 } else { 0 }; 5],
                [if max_value > avg + 80 / 2 { 1 } else { 0 }; 5],
                [if max_value > avg + 60 / 2 { 1 } else { 0 }; 5],
                [if max_value > avg + 40 / 2 { 1 } else { 0 }; 5],
                [if max_value > avg + 20 / 2 { 1 } else { 0 }; 5],
            ];
            display.show(&mut timer, image, 100);
            max_value = 0;
        }
    }
}
