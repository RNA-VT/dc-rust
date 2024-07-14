#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_hal::hal::port::{PE0, PE1, PE4, PJ0, PJ1};
use arduino_hal::pac::USART0;
use arduino_hal::pac::USART3;
use arduino_hal::port;
use arduino_hal::port::mode::{Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::Usart;
use embedded_hal::serial::Read;
use max485::Max485;
use ufmt::uWrite;

use hotline::hotline::parse_command;

type UsartType = Usart<USART3, port::Pin<Input, PJ0>, port::Pin<Output, PJ1>>;
type Max485Type = Max485<UsartType, port::Pin<Output, PE4>>;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut pin_solenoid_1 = pins.d22.into_output();
    let mut pin_solenoid_2 = pins.d24.into_output();
    let mut pin_solenoid_3 = pins.d26.into_output();
    let mut pin_pilot = pins.d28.into_output();

    // RS485 digital output pin
    let mut pin_rs485_enable = pins.d2.into_output();
    pin_rs485_enable.set_low();

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
    let mut states: [u8; 12] = [0; 12];
    loop {
        // Read a byte from the serial connection
        match receive_command(&mut rs485, &mut usb) {
            Some((device_id, dio_id, state)) => {
                if device_id == 0x01 {
                    if dio_id == 0xFF {
                        if state == 0x01 {
                            pin_solenoid_1.set_low();
                            pin_solenoid_2.set_low();
                            pin_solenoid_3.set_low();
                        } else {
                            pin_solenoid_1.set_high();
                            pin_solenoid_2.set_high();
                            pin_solenoid_3.set_high();
                        }
                    } else if dio_id < 4 {
                        if states[dio_id as usize] != state {
                            match dio_id {
                                0 => {
                                    if state == 0x01 {
                                        pin_solenoid_1.set_low();
                                    } else {
                                        pin_solenoid_1.set_high();
                                    }
                                }
                                1 => {
                                    if state == 0x01 {
                                        pin_solenoid_2.set_low();
                                    } else {
                                        pin_solenoid_2.set_high();
                                    }
                                }
                                2 => {
                                    if state == 0x01 {
                                        pin_solenoid_3.set_low();
                                    } else {
                                        pin_solenoid_3.set_high();
                                    }
                                }
                                3 => {
                                    if state == 0x01 {
                                        pin_pilot.set_low();
                                    } else {
                                        pin_pilot.set_high();
                                    }
                                }
                                _ => {
                                    usb.write_str("Invalid DIO ID Received").unwrap();
                                }
                            }
                            states[dio_id as usize] = state;
                        }
                    } else {
                        usb.write_str("Invalid DIO ID Received").unwrap();
                    }
                    usb.write_str("Received command - ").unwrap();
                    ufmt::uwrite!(
                        usb,
                        "Device ID: {:X}, DIO ID: {:X}, State: {:X}\n",
                        device_id,
                        dio_id,
                        state
                    )
                    .unwrap();
                }
            }
            None => {
                usb.write_str("Failed to parse command.\n").unwrap();
            }
        }
    }
}
fn receive_command(
    serial: &mut Max485Type,
    usb: &mut Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>>,
) -> Option<(u8, u8, u8)> {
    let mut buffer = [0u8; 9];
    let mut index = 0;

    loop {
        let byte = nb::block!(serial.read()).unwrap();
        // Wait for the start delimiter
        if (index == 0 && byte == 0xFF) || (index == 1 && buffer[0] == 0xFF && byte == 0x00) {
            buffer[index] = byte;
            index += 1;
        } else if index > 1 {
            buffer[index] = byte;
            index += 1;
            // If the buffer is full, try to parse the command
            if index == 9 {
                return parse_command(&buffer, usb);
            }
        } else {
            // Reset if the start delimiter is not correct
            index = 0;
        }
    }
}
