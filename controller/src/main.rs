#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_hal::hal::port::{PE4, PJ0, PJ1};
use arduino_hal::port;
use arduino_hal::port::mode::{Input, Output};
use arduino_hal::prelude::*;
use max485::Max485;

use hotline::hotline::create_command;
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let pin_input = pins.d13.into_pull_up_input();

    // Pins and their corresponding addresses
    let pin_addresses = [
        (pins.d31.into_pull_up_input(), 0xFF), //Broadcast
        (pins.d22.into_pull_up_input(), 0x00),
        (pins.d24.into_pull_up_input(), 0x01),
        (pins.d26.into_pull_up_input(), 0x02),
        (pins.d28.into_pull_up_input(), 0x03),
        (pins.d30.into_pull_up_input(), 0x04),
        (pins.d32.into_pull_up_input(), 0x05),
        (pins.d34.into_pull_up_input(), 0x06),
        (pins.d36.into_pull_up_input(), 0x07),
        (pins.d38.into_pull_up_input(), 0x08),
        (pins.d40.into_pull_up_input(), 0x09),
        (pins.d42.into_pull_up_input(), 0x10),
        (pins.d44.into_pull_up_input(), 0x12),
    ];

    // RS485 digital output pin
    let mut pin_rs485_enable = pins.d2.into_output();
    pin_rs485_enable.set_high();

    // Initialize USB serial for debugging
    let mut usb = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(57600), // USB
    );

    // Initialize RS485 serial communication
    let serial = arduino_hal::Usart::new(
        dp.USART3,
        pins.d15,
        pins.d14.into_output(),
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(460800), // RS485
    );

    // Max485 initialization
    let mut rs485 = Max485::new(serial, pin_rs485_enable);

    loop {
        rs485.flush().unwrap();
        usb.write_str("In loop\n").unwrap();

        for (pin, address) in pin_addresses.iter() {
            if pin.is_high() {
                send_command(&mut rs485, 0x00, *address, 0x01).unwrap();
            } else {
                send_command(&mut rs485, 0x00, *address, 0x00).unwrap();
            }
        }
    }
}

type UsartType =
    arduino_hal::Usart<arduino_hal::pac::USART3, port::Pin<Input, PJ0>, port::Pin<Output, PJ1>>;
type Max485Type = Max485<UsartType, port::Pin<Output, PE4>>;
fn send_command(serial: &mut Max485Type, device_id: u8, dio_id: u8, state: u8) -> Result<(), ()> {
    // Create command
    let command = create_command(device_id, dio_id, state);

    // Send command bytesI'
    for byte in &command {
        match serial.write(*byte) {
            Ok(()) => {}
            Err(_) => return Err(()),
        };
        arduino_hal::delay_us(100);
    }

    Ok(())
}
