#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_hal::hal::port::{PE4, PJ0, PJ1};
use arduino_hal::port;
use arduino_hal::port::mode::{Input, Output};
use arduino_hal::prelude::*;
use max485::Max485;

use hotline::hotline::create_command;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Initialize all pins
    let pin_all = pins.d31.into_pull_up_input();
    let pin_input_1 = pins.d22.into_pull_up_input();
    let pin_input_2 = pins.d24.into_pull_up_input();
    let pin_input_3 = pins.d26.into_pull_up_input();
    let pin_input_4 = pins.d28.into_pull_up_input();
    let pin_input_5 = pins.d30.into_pull_up_input();
    let pin_input_6 = pins.d32.into_pull_up_input();
    let pin_input_7 = pins.d34.into_pull_up_input();
    let pin_input_8 = pins.d36.into_pull_up_input();
    let pin_input_9 = pins.d38.into_pull_up_input();
    let pin_input_10 = pins.d40.into_pull_up_input();
    let pin_input_11 = pins.d42.into_pull_up_input();
    let pin_input_12 = pins.d44.into_pull_up_input();

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

        if pin_all.is_high() {
            send_command(&mut rs485, 0x00, 0xFF, 0x01).unwrap();
            continue;
        } else {
            send_command(&mut rs485, 0x00, 0xFF, 0x00).unwrap();
        }

        if pin_input_1.is_high() {
            send_command(&mut rs485, 0x00, 0x00, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x00, 0x00).unwrap();
        }
        if pin_input_2.is_high() {
            send_command(&mut rs485, 0x00, 0x01, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x01, 0x00).unwrap();
        }
        if pin_input_3.is_high() {
            send_command(&mut rs485, 0x00, 0x02, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x02, 0x00).unwrap();
        }
        if pin_input_4.is_high() {
            send_command(&mut rs485, 0x00, 0x03, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x03, 0x00).unwrap();
        }
        if pin_input_5.is_high() {
            send_command(&mut rs485, 0x00, 0x04, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x04, 0x00).unwrap();
        }
        if pin_input_6.is_high() {
            send_command(&mut rs485, 0x00, 0x05, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x05, 0x00).unwrap();
        }
        if pin_input_7.is_high() {
            send_command(&mut rs485, 0x00, 0x06, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x06, 0x00).unwrap();
        }
        if pin_input_8.is_high() {
            send_command(&mut rs485, 0x00, 0x07, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x07, 0x00).unwrap();
        }
        if pin_input_9.is_high() {
            send_command(&mut rs485, 0x00, 0x08, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x08, 0x00).unwrap();
        }
        if pin_input_10.is_high() {
            send_command(&mut rs485, 0x00, 0x09, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x09, 0x00).unwrap();
        }
        if pin_input_11.is_high() {
            send_command(&mut rs485, 0x00, 0x10, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x10, 0x00).unwrap();
        }
        if pin_input_12.is_high() {
            send_command(&mut rs485, 0x00, 0x11, 0x01).unwrap();
        } else {
            send_command(&mut rs485, 0x00, 0x11, 0x00).unwrap();
        }
    }
}

type UsartType =
    arduino_hal::Usart<arduino_hal::pac::USART3, port::Pin<Input, PJ0>, port::Pin<Output, PJ1>>;
type Max485Type = Max485<UsartType, port::Pin<Output, PE4>>;
fn send_command(serial: &mut Max485Type, device_id: u8, dio_id: u8, state: u8) -> Result<(), ()> {
    // Create command
    let command = create_command(device_id, dio_id, state);

    // Send command bytes
    for byte in &command {
        match serial.write(*byte) {
            Ok(()) => {}
            Err(_) => return Err(()),
        };
        arduino_hal::delay_us(100);
    }

    Ok(())
}
