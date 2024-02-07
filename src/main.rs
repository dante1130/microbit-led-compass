#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::Board;
use panic_rtt_target as _;
use rtt_target::rtt_init_default;

#[entry]
fn main() -> ! {
    rtt_init_default!();

    let mut board = match Board::take() {
        Some(board) => board,
        None => panic!("Microbit not available or already taken."),
    };

    match board.display_pins.col1.set_low() {
        Ok(_) => {}
        Err(_) => panic!("Error setting col1 low."),
    }

    match board.display_pins.row1.set_high() {
        Ok(_) => {}
        Err(_) => panic!("Error setting row1 high."),
    }

    loop {}
}
