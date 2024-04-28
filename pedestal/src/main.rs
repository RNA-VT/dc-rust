#![no_std]
#![no_main]

use arduino_hal::hal::port::PE4;
use panic_halt as _;
use arduino_hal::prelude::*;
use arduino_hal::hal::port::mode::Output;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Pins for solenoids
    let mut pin_solenoid_1 = pins.d22.into_pull_up_input();
    let mut pin_solenoid_2 = pins.d24.into_pull_up_input();
    let mut pin_solenoid_3 = pins.d26.into_pull_up_input();
    let mut pin_solenoid_4 = pins.d28.into_pull_up_input();
    let mut pin_solenoid_5 = pins.d30.into_pull_up_input();
    let mut pin_solenoid_all = pins.d31.into_pull_up_input();

    // RS485 digital output pin
    let mut pin_rs485_do = pins.d2.into_output();

    // Array to store the status of solenoids
    let mut solenoid_states = [false; 5];

    loop {
        if pin_solenoid_all.is_high() {
            solenoid_states = [true; 5];
        } else {
            solenoid_states[0] = pin_solenoid_1.is_high();
            solenoid_states[1] = pin_solenoid_2.is_high();
            solenoid_states[2] = pin_solenoid_3.is_high();
            solenoid_states[3] = pin_solenoid_4.is_high();
            solenoid_states[4] = pin_solenoid_5.is_high();
        }

        // Send RS485 message with start and end symbols
        send_rs485_message(&mut pin_rs485_do, &solenoid_states).unwrap(); // Handle errors as needed
        arduino_hal::delay_ms(100);
    }
}

/// Simulates sending an RS485 message by toggling the DO pin based on solenoid states.
fn send_rs485_message(pin_rs485_do: &mut arduino_hal::port::Pin<Output, PE4>, solenoid_states: &[bool; 5]) -> Result<(), &'static str> {
    // Send start symbol (0b01111110)
    send_symbol(pin_rs485_do, 0b01111110);

    // Encode solenoid states into a single byte
    let mut state_byte: u8 = 0;
    for (i, &state) in solenoid_states.iter().enumerate() {
        if state {
            state_byte |= 1 << i;
        }
    }

    // Send the state byte
    send_symbol(pin_rs485_do, state_byte);

    // Send end symbol (0b01111110)
    send_symbol(pin_rs485_do, 0b01111110);

    Ok(())
}

/// Sends a byte as individual bits to the RS485 bus
fn send_symbol(pin_rs485_do: &mut arduino_hal::port::Pin<Output, PE4>, symbol: u8) {
    for i in 0..8 {
        if (symbol >> i) & 1 == 1 {
            pin_rs485_do.set_high();
        } else {
            pin_rs485_do.set_low();
        }
        arduino_hal::delay_ms(10);  // Bit duration
    }
}