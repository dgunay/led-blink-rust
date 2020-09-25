#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB4;
use arduino_uno::prelude::*;

fn blink(led: &mut PB4<Output>, times: usize) {
    for _ in 0..times {
        // turn LED on
        led.toggle().void_unwrap();

        // wait
        arduino_uno::delay_ms(250);

        // turn LED off
        led.toggle().void_unwrap();

        // wait
        arduino_uno::delay_ms(250);
    }
}

const BAUD_RATE: u32 = 57600;

#[arduino_uno::entry]
fn main() -> ! {
    // This first part corresponds to the C Arduino language "setup()" function.

    // Take the Peripherals (things connected to the pins on the chip).
    // We'll mainly care about the one corresponding to the pin we've plugged
    // our LED into.
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

    // Typestate idiom ensures we can only use output-enabled pins.
    let mut led = pins.d12.into_output(&mut pins.ddr);

    // This is just so we can print debug messages. TODO: compile this out in
    // release
    let mut serial = arduino_uno::Serial::new(
        peripherals.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        BAUD_RATE,
    );

    ufmt::uwriteln!(&mut serial, "hi").void_unwrap();

    // This part corresponds to the C Arduino language "loop()" function.
    loop {
        blink(&mut led, 3);
        arduino_uno::delay_ms(10000);
    }
}
