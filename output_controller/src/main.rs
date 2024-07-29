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
    let mut pin_output_0 = pins.d22.into_output();
    let mut pin_output_1 = pins.d24.into_output();
    let mut pin_output_2 = pins.d26.into_output();
    let mut pin_output_3 = pins.d28.into_output();
    let mut pin_output_4 = pins.d30.into_output();
    let mut pin_output_5 = pins.d32.into_output();
    let mut pin_output_6 = pins.d34.into_output();
    let mut pin_output_7 = pins.d36.into_output();
    

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
    usb.write_str("Serial Initialized\n").unwrap();

    let mut states: [bool; 4] = [false; 4];
    loop {
        // Read a byte from the serial connection
        match receive_command(&mut rs485, &mut usb) {
            Some(msg) => {
                let device_id = msg.device_id;
                // 0x00 - Sign, 0x01 - MegaPoofer, 0xFF - All outputs
                if device_id == 0x01 || device_id == 0x00 || device_id == 0xFF  {
                    if let Some(state) = msg.get_dio_state(0) {
                        if state != states[0] {
                            if state {
                                usb.write_str("Output 0 High\n").unwrap();
                                pin_output_0.set_high();
                            } else {
                                usb.write_str("Output 0 Low\n").unwrap();
                                pin_output_0.set_low();
                            }
                            states[0] = state;
                        }
                    }

                    if let Some(state) = msg.get_dio_state(1) {
                        if state != states[1] {
                            if state {
                                usb.write_str("Output 1 High\n").unwrap();
                                pin_output_1.set_high();
                            } else {
                                usb.write_str("Output 1 Low\n").unwrap();
                                pin_output_1.set_low();
                            }
                            states[1] = state;
                        }
                    }

                    if let Some(state) = msg.get_dio_state(2) {
                        if state != states[2] {
                            if state {
                                usb.write_str("Output 2 High\n").unwrap();
                                pin_output_2.set_high();
                            } else {
                                usb.write_str("Output 2 Low\n").unwrap();
                                pin_output_2.set_low();
                            }
                            states[2] = state;
                        }
                    }

                    if let Some(state) = msg.get_dio_state(3) {
                        if state != states[3] {
                            if state {
                                usb.write_str("Output 3 High\n").unwrap();
                                pin_output_3.set_high();
                            } else {
                                usb.write_str("Output 3 Low\n").unwrap();
                                pin_output_3.set_low();
                            }
                            states[3] = state;
                        }
                    }
                }

                if device_id == 0x00 || device_id == 0xFF {
                    if let Some(state) = msg.get_dio_state(4) {
                        if state != states[4] {
                            if state {
                                usb.write_str("Output 4 High\n").unwrap();
                                pin_output_4.set_high();
                            } else {
                                usb.write_str("Output 4 Low\n").unwrap();
                                pin_output_4.set_low();
                            }
                            states[4] = state;
                        }
                    }

                    if let Some(state) = msg.get_dio_state(5) {
                        if state != states[5] {
                            if state {
                                usb.write_str("Output 5 High\n").unwrap();
                                pin_output_5.set_high();
                            } else {
                                usb.write_str("Output 5 Low\n").unwrap();
                                pin_output_5.set_low();
                            }
                            states[5] = state;
                        }
                    }
                }

                if device_id == 0xFF {
                    if let Some(state) = msg.get_dio_state(6) {
                        if state != states[6] {
                            if state {
                                usb.write_str("Output 6 High\n").unwrap();
                                pin_output_6.set_high();
                            } else {
                                usb.write_str("Output 6 Low\n").unwrap();
                                pin_output_6.set_low();
                            }
                            states[6] = state;
                        }
                    }

                    if let Some(state) = msg.get_dio_state(7) {
                        if state != states[7] {
                            if state {
                                usb.write_str("Output 7 High\n").unwrap();
                                pin_output_7.set_high();
                            } else {
                                usb.write_str("Output 7 Low\n").unwrap();
                                pin_output_7.set_low();
                            }
                            states[7] = state;
                        }
                    }
                }
                    usb.write_str("Received command - ").unwrap();
                    ufmt::uwrite!(
                        usb,
                        "Device ID: {:X}, States: {:?}\n",
                        msg.device_id,
                        states,
                    ).unwrap();
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
            index = 0;
            buffer = [0u8; 8];
        }
    }
}
