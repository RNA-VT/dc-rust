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

use hotline::hotline_protocol::{HotlineMessage, ParseError};

type UsartType = Usart<USART3, port::Pin<Input, PJ0>, port::Pin<Output, PJ1>>;
type Max485Type = Max485<UsartType, port::Pin<Output, PE4>>;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(dp);
    let mut pin_output_0 = pins.d12.into_output();
    let mut pin_output_1 = pins.d7.into_output();
    let mut pin_output_2 = pins.d8.into_output();
    let mut pin_output_3 = pins.d9.into_output();
    let mut pin_output_4 = pins.d10.into_output();
    let mut pin_output_5 = pins.d13.into_output();
    let mut pin_output_6 = pins.d5.into_output();
    let mut pin_output_7 = pins.d6.into_output();

    // turn off all outputs
    pin_output_0.set_high();
    pin_output_1.set_high();
    pin_output_2.set_high();
    pin_output_3.set_high();
    pin_output_4.set_high();
    pin_output_5.set_high();
    pin_output_6.set_high();
    pin_output_7.set_high();

    // RS485 digital output pin
    let mut pin_rs485_enable = pins.d2.into_output();
    pin_rs485_enable.set_high(); //tested

    // USB
    let mut usb = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(57600), // USB
    );

    // RS485
    let serial = arduino_hal::Usart::new(
        dp.USART3,
        pins.d15,
        pins.d14.into_output(),
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(460800), // RS485
    );

    // Max485 crate
    let mut rs485 = Max485::new(serial, pin_rs485_enable);
    usb.write_str("Serial Initialized\n").unwrap();

    let mut states: [bool; 8] = [false; 8];

    loop {
        match receive_command(&mut rs485, &mut usb) {
            Some(msg) => {
                let device_id = msg.device_id;
                usb.write_str("Message received\n").unwrap();
                // 0x00 - Sign, 0x01 - MegaPoofer, 0xFF - All outputs
                if device_id == 0x01 || device_id == 0x00 || device_id == 0xFF {
                    if let Some(state) = msg.get_dio_state(0) {
                        if state != states[0] {
                            if state {
                                pin_output_0.set_low();
                            } else {
                                pin_output_0.set_high();
                            }
                            states[0] = state;
                        }
                    }

                    if let Some(state) = msg.get_dio_state(1) {
                        if state != states[1] {
                            if state {
                                pin_output_1.set_low();
                            } else {
                                pin_output_1.set_high();
                            }
                            states[1] = state;
                        }
                    }

                    if let Some(state) = msg.get_dio_state(2) {
                        if state != states[2] {
                            if state {
                                pin_output_2.set_low();
                            } else {
                                pin_output_2.set_high();
                            }
                            states[2] = state;
                        }
                    }

                    if let Some(state) = msg.get_dio_state(3) {
                        if state != states[3] {
                            if state {
                                pin_output_3.set_low();
                            } else {
                                pin_output_3.set_high();
                            }
                            states[3] = state;
                        }
                    }
                }
                if device_id == 0x00 || device_id == 0xFF {
                    if let Some(state) = msg.get_dio_state(4) {
                        if state != states[4] {
                            if state {
                                pin_output_4.set_low();
                            } else {
                                pin_output_4.set_high();
                            }
                            states[4] = state;
                        }
                    }

                    if let Some(state) = msg.get_dio_state(5) {
                        if state != states[5] {
                            if state {
                                pin_output_5.set_low();
                            } else {
                                pin_output_5.set_high();
                            }
                            states[5] = state;
                        }
                    }
                }
                if device_id == 0xFF {
                    if let Some(state) = msg.get_dio_state(6) {
                        if state != states[6] {
                            if state {
                                pin_output_6.set_low();
                            } else {
                                pin_output_6.set_high();
                            }
                            states[6] = state;
                        }
                    }

                    if let Some(state) = msg.get_dio_state(7) {
                        if state != states[7] {
                            if state {
                                pin_output_7.set_low();
                            } else {
                                pin_output_7.set_high();
                            }
                            states[7] = state;
                        }
                    }
                }
                // ufmt::uwrite!(
                //     usb,
                //     "Device ID: {:X}, States: {:?}\n",
                //     msg.device_id,
                //     states,
                // )
                // .unwrap();
            }
            _ => {
                usb.write_str("Failed to parse command.\n").unwrap();
            }
        }
    }
}

fn receive_command(
    serial: &mut Max485Type,
    usb: &mut Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>>,
) -> Option<HotlineMessage> {
    let mut buffer = [0u8; 8];
    let mut index = 0;

    loop {
        let byte = nb::block!(serial.read()).unwrap();
        // ufmt::uwrite!(usb, "Byte Received: {:X}\n", byte).unwrap();
        // Wait for the start delimiter
        if (index == 0 && byte == 0xBE) || (index == 1 && buffer[0] == 0xBE && byte == 0xEF) {
            buffer[index] = byte;
            index += 1;
        } else if index > 1 {
            buffer[index] = byte;
            index += 1;
            // If the buffer is full, try to parse the command
            if index == 8 {
                match HotlineMessage::from_bytes(&buffer) {
                    Ok(msg) => {
                        return Some(msg);
                    }
                    Err(e) => {
                        usb.write_str("Failed to Parse Message\n").unwrap();
                        match e {
                            ParseError::InvalidLength => {
                                usb.write_str("Invalid Length\n").unwrap();
                            }
                            ParseError::InvalidDelimiters => {
                                usb.write_str("Invalid Delimiters\n").unwrap();
                            }
                            ParseError::InvalidCrc => {
                                usb.write_str("Invalid CRC\n").unwrap();
                            }
                        }
                    }
                };
                index = 0;
                buffer = [0u8; 8];
            }
        } else {
            usb.write_str("Stream Parse Fail\n").unwrap();
            index = 0;
            buffer = [0u8; 8];
        }
    }
}
