#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::f32::consts::PI;
use cortex_m_rt::entry;
use libm::atan2f;
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

        let radians = atan2f(calibrated_mag_data.y as f32, calibrated_mag_data.x as f32);

        let degrees = radians * 180.0 / PI;

        let direction = if degrees > 120.0 && degrees <= 150.0 {
            Direction::NorthWest
        } else if degrees > 60.0 && degrees <= 120.0 {
            Direction::North
        } else if degrees > 30.0 && degrees <= 60.0 {
            Direction::NorthEast
        } else if degrees > -30.0 && degrees <= 30.0 {
            Direction::East
        } else if degrees > -60.0 && degrees <= -30.0 {
            Direction::SouthEast
        } else if degrees > -120.0 && degrees <= -60.0 {
            Direction::South
        } else if degrees > -150.0 && degrees <= -120.0 {
            Direction::SouthWest
        } else {
            Direction::West
        };

        let led_matrix = direction_led::get_led_matrix(direction);

        display.show(&mut timer, led_matrix, 100);
    }
}
