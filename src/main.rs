#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr, MagMode, MagOutputDataRate};
use microbit::display::blocking::Display;
use microbit::hal::twim::Frequency;
use microbit::hal::{Delay, Timer, Twim};
use microbit::Board;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use crate::calibration::Measurement;

mod calibration;
mod direction_led;

use direction_led::Direction;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = match Board::take() {
        Some(board) => board,
        None => panic!("Microbit not available or already taken."),
    };

    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), Frequency::K100);

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut delay = Delay::new(board.SYST);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_mag_mode_and_odr(&mut delay, MagMode::HighResolution, MagOutputDataRate::Hz10)
        .unwrap();
    sensor
        .set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz10)
        .unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let calibration = calibration::calc_calibration(&mut sensor, &mut display, &mut timer);
    rprintln!("Calibration: {:?}", calibration);
    rprintln!("Calibration done.");

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data() {}

        let mag_data = sensor.magnetic_field().unwrap();

        let calibrated_mag_data = calibration::calibrated_measurement(
            Measurement {
                x: mag_data.x_nt(),
                y: mag_data.y_nt(),
                z: mag_data.z_nt(),
            },
            &calibration,
        );

        let direction = match (calibrated_mag_data.x > 0, calibrated_mag_data.y > 0) {
            (true, true) => Direction::NorthEast,
            (true, false) => Direction::SouthEast,
            (false, true) => Direction::NorthWest,
            (false, false) => Direction::SouthWest,
        };

        let led_matrix = direction_led::get_led_matrix(direction);

        display.show(&mut timer, led_matrix, 100);
    }
}
