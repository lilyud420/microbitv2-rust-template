#![no_std]
#![no_main]

/*
cortex-m
cortex-m-rt
embedded-hal
microbit-v2
panic-rtt-target
rtt-target
*/

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::hal::{gpio, timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;


// Example: blinking led
#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let mut timer = timer::Timer::new(board.TIMER0);

    let mut row3 = board
        .display_pins
        .row3
        .into_push_pull_output(gpio::Level::High);

    let mut _col3 = board
        .display_pins
        .col3
        .into_push_pull_output(gpio::Level::Low);

    loop {
        row3.set_high().unwrap();
        timer.delay_ms(500);
        row3.set_low().unwrap();
        timer.delay_ms(500);
    }
}
