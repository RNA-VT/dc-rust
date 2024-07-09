#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_hal::prelude::*;
use max485::Max485;
use core::fmt::{Debug, Write};
use arduino_hal::DefaultClock;
use arduino_hal::hal::Atmega;
use arduino_hal::hal::port::{PE0, PE1, PJ0, PJ1};
use arduino_hal::hal::usart::Usart0;
use arduino_hal::pac::{USART0, USART3};
use arduino_hal::port::mode::{Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::Usart;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);


    let pin_input = pins.d13.into_pull_up_input();
    // Pins for solenoids
    let pin_solenoid_1 = pins.d22.into_pull_up_input();
    let pin_solenoid_2 = pins.d24.into_pull_up_input();
    let pin_solenoid_3 = pins.d26.into_pull_up_input();
    let pin_solenoid_4 = pins.d28.into_pull_up_input();
    let pin_solenoid_5 = pins.d30.into_pull_up_input();
    let pin_solenoid_all = pins.d31.into_pull_up_input();

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
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(256000), // RS485
    );

    // Max485 initialization
    let mut rs485 = Max485::new(serial, pin_rs485_enable);

    // Set Baud Rate initialization message
    usb.write_str("Set Baud Rate RS485\n").unwrap();
    let mut init_cmd: [u8; 8] = [0x00, 0x06, 0x20, 0x00, 0x00, 0x07, 0x5C, 0x1b];
    let crc = modbus_crc(&init_cmd[..6]);
    init_cmd[6] = (crc & 0xFF) as u8;
    init_cmd[7] = (crc >> 8) as u8;

    // Convert the cmd array to a string of hex values
    let mut buffer = [0u8; 32]; // Adjust the size as needed
    let cmd_str = HexFormatter::bytes_to_hex_string(&init_cmd, &mut buffer);
    usb.write_str(cmd_str).unwrap();

    let mut buffer1 = [0u8; 32]; // Adjust the size as needed
    let cmd_str1 = HexFormatter::bytes_to_hex_string(&mut init_cmd, &mut buffer1);
    usb.write_str(cmd_str1).unwrap();

    for byte in &mut init_cmd {
        match rs485.write(*byte) {
            Ok(()) => {
                usb.write_str(".").unwrap()
            }
            Err(e) => {
                let mut error_buffer = [0u8; 64];
                let mut output_buffer = [0u8; 64];
                let error_str = format_error_string(e, &mut error_buffer, &mut output_buffer);
                usb.write_str(error_str.into()).unwrap();
                usb.write_str("\n").unwrap();
            }
        }
        arduino_hal::delay_us(600);
    }

    // Send initialization message
    usb.write_str("Set Device Address\n").unwrap();
    let mut init_cmd: [u8; 8] = [0x00, 0x06, 0x40, 0x00, 0x00, 0x01, 0x5C, 0x1b];
    let crc = modbus_crc(&init_cmd[..6]);
    init_cmd[6] = (crc & 0xFF) as u8;
    init_cmd[7] = (crc >> 8) as u8;

    let mut buffer1 = [0u8; 32]; // Adjust the size as needed
    let cmd_str1 = HexFormatter::bytes_to_hex_string(&mut init_cmd, &mut buffer1);
    usb.write_str(cmd_str1).unwrap();

    for byte in &mut init_cmd {
        match rs485.write(*byte) {
            Ok(()) => {
                usb.write_str(".").unwrap()
            }
            Err(e) => {
                let mut error_buffer = [0u8; 64];
                let mut output_buffer = [0u8; 64];
                let error_str = format_error_string(e, &mut error_buffer, &mut output_buffer);
                usb.write_str(error_str.into()).unwrap();
                usb.write_str("\n").unwrap();
            }
        }
        arduino_hal::delay_us(600);
    }

    // Delay for stability
    arduino_hal::delay_ms(100);

    // Base relay command setup
    let mut cmd: [u8; 8] = [
        0x00, // Device Address
        0x05, // Command Relay
        0x00, // Always 0
        0x00, // Relay ID 0x00 - 0x08
        0xFF, // Relay Open/Close - 0xFF Open, 0x00 Close
        0x00, // Always 0
        0x00, // CRC16 byte 0
        0x00  // CRC16 byte 1
    ];
    let mut high = true;
    loop {
        usb.write_str("In loop\n").unwrap();
        cmd[3] = 0xFF;
        cmd[4] = 0x55;
        let crc = modbus_crc(&cmd[..6]);
        cmd[6] = (crc & 0xFF) as u8;
        cmd[7] = (crc >> 8) as u8;

        for i in 0..8 {
            rs485.flush().unwrap();
            cmd[3] = i as u8;
            if high {
                cmd[4]=0xFF;
            } else {
                cmd[4]=0x00;
            }

            let crc = modbus_crc(&cmd[..6]);
            cmd[6] = (crc & 0xFF) as u8;
            cmd[7] = (crc >> 8) as u8;

            // Convert the cmd array to a string of hex values
            let mut buffer = [0u8; 32]; // Adjust the size as needed
            let cmd_str = HexFormatter::bytes_to_hex_string(&cmd, &mut buffer);
            usb.write_str("Sending Command: ").unwrap();
            usb.write_str(cmd_str).unwrap();
            let mut buffer1 = [0u8; 32]; // Adjust the size as needed
            let cmd_str1 = HexFormatter::bytes_to_hex_string(&mut cmd, &mut buffer1);
            usb.write_str(cmd_str1).unwrap();

            for byte in &mut cmd {
                match rs485.write(*byte) {
                    Ok(()) => {
                        usb.write_str(".").unwrap()
                    }
                    Err(e) => {
                        let mut error_buffer = [0u8; 64];
                        let mut output_buffer = [0u8; 64];
                        let error_str = format_error_string(e, &mut error_buffer, &mut output_buffer);
                        usb.write_str(error_str.into()).unwrap();
                        usb.write_str("\n").unwrap();
                    }
                }
                arduino_hal::delay_us(600);
            }
        }
        arduino_hal::delay_ms(10);
        usb.write_str("\n").unwrap();
        high = !high;

        if pin_input.is_high() {
            usb.write_str("Channel 1 High\n").unwrap();
        } else {
            usb.write_str("Channel 1 Low\n").unwrap();
        }
        arduino_hal::delay_ms(62);
    }
}

pub fn modbus_crc(data: &[u8]) -> u16 {
    // Modbus CRC high byte lookup table
    const CRC_TABLE_HIGH: [u8; 256] = [
        0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41, 0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40,
        0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40, 0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41,
        0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40, 0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41,
        0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41, 0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40,
        0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40, 0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41,
        0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41, 0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40,
        0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41, 0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40,
        0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40, 0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41,
        0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40, 0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41,
        0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41, 0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40,
        0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41, 0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40,
        0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40, 0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41,
        0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41, 0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40,
        0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40, 0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41,
        0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40, 0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41,
        0x00, 0xC1, 0x81, 0x40, 0x01, 0xC0, 0x80, 0x41, 0x01, 0xC0, 0x80, 0x41, 0x00, 0xC1, 0x81, 0x40
    ];

    // Modbus CRC low byte lookup table
    const CRC_TABLE_LOW: [u8; 256] = [
        0x00, 0xC0, 0xC1, 0x01, 0xC3, 0x03, 0x02, 0xC2, 0xC6, 0x06, 0x07, 0xC7, 0x05, 0xC5, 0xC4, 0x04,
        0xCC, 0x0C, 0x0D, 0xCD, 0x0F, 0xCF, 0xCE, 0x0E, 0x0A, 0xCA, 0xCB, 0x0B, 0xC9, 0x09, 0x08, 0xC8,
        0xD8, 0x18, 0x19, 0xD9, 0x1B, 0xDB, 0xDA, 0x1A, 0x1E, 0xDE, 0xDF, 0x1F, 0xDD, 0x1D, 0x1C, 0xDC,
        0x14, 0xD4, 0xD5, 0x15, 0xD7, 0x17, 0x16, 0xD6, 0xD2, 0x12, 0x13, 0xD3, 0x11, 0xD1, 0xD0, 0x10,
        0xF0, 0x30, 0x31, 0xF1, 0x33, 0xF3, 0xF2, 0x32, 0x36, 0xF6, 0xF7, 0x37, 0xF5, 0x35, 0x34, 0xF4,
        0x3C, 0xFC, 0xFD, 0x3D, 0xFF, 0x3F, 0x3E, 0xFE, 0xFA, 0x3A, 0x3B, 0xFB, 0x39, 0xF9, 0xF8, 0x38,
        0x28, 0xE8, 0xE9, 0x29, 0xEB, 0x2B, 0x2A, 0xEA, 0xEE, 0x2E, 0x2F, 0xEF, 0x2D, 0xED, 0xEC, 0x2C,
        0xE4, 0x24, 0x25, 0xE5, 0x27, 0xE7, 0xE6, 0x26, 0x22, 0xE2, 0xE3, 0x23, 0xE1, 0x21, 0x20, 0xE0,
        0xA0, 0x60, 0x61, 0xA1, 0x63, 0xA3, 0xA2, 0x62, 0x66, 0xA6, 0xA7, 0x67, 0xA5, 0x65, 0x64, 0xA4,
        0x6C, 0xAC, 0xAD, 0x6D, 0xAF, 0x6F, 0x6E, 0xAE, 0xAA, 0x6A, 0x6B, 0xAB, 0x69, 0xA9, 0xA8, 0x68,
        0x78, 0xB8, 0xB9, 0x79, 0xBB, 0x7B, 0x7A, 0xBA, 0xBE, 0x7E, 0x7F, 0xBF, 0x7D, 0xBD, 0xBC, 0x7C,
        0xB4, 0x74, 0x75, 0xB5, 0x77, 0xB7, 0xB6, 0x76, 0x72, 0xB2, 0xB3, 0x73, 0xB1, 0x71, 0x70, 0xB0,
        0x50, 0x90, 0x91, 0x51, 0x93, 0x53, 0x52, 0x92, 0x96, 0x56, 0x57, 0x97, 0x55, 0x95, 0x94, 0x54,
        0x9C, 0x5C, 0x5D, 0x9D, 0x5F, 0x9F, 0x9E, 0x5E, 0x5A, 0x9A, 0x9B, 0x5B, 0x99, 0x59, 0x58, 0x98,
        0x88, 0x48, 0x49, 0x89, 0x4B, 0x8B, 0x8A, 0x4A, 0x4E, 0x8E, 0x8F, 0x4F, 0x8D, 0x4D, 0x4C, 0x8C,
        0x44, 0x84, 0x85, 0x45, 0x87, 0x47, 0x46, 0x86, 0x82, 0x42, 0x43, 0x83, 0x41, 0x81, 0x80, 0x40
    ];

    let mut crc_high = 0xFF;
    let mut crc_low = 0xFF;

    for byte in data {
        let index = (crc_low ^ byte) as usize;
        crc_low = crc_high ^ CRC_TABLE_HIGH[index];
        crc_high = CRC_TABLE_LOW[index];
    }

    ((crc_high as u16) << 8) | (crc_low as u16)
}

struct HexFormatter;

impl HexFormatter {
    fn byte_to_hex(byte: u8) -> (char, char) {
        const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";
        (
            HEX_DIGITS[(byte >> 4) as usize] as char,
            HEX_DIGITS[(byte & 0x0F) as usize] as char,
        )
    }

    fn bytes_to_hex_string<'a>(buf: &[u8], output: &'a mut [u8]) -> &'a str {
        let mut index = 0;
        for &byte in buf {
            if index > 0 {
                output[index] = b' ';
                index += 1;
            }
            let (high, low) = Self::byte_to_hex(byte);
            output[index] = high as u8;
            output[index + 1] = low as u8;
            index += 2;
        }
        core::str::from_utf8(&output[..index]).unwrap()
    }
}
struct SimpleWriter<'a> {
    buffer: &'a mut [u8],
    index: usize,
}

impl<'a> SimpleWriter<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, index: 0 }
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buffer[..self.index]).unwrap()
    }
}

impl<'a> Write for SimpleWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len();

        if self.index + len <= self.buffer.len() {
            self.buffer[self.index..self.index + len].copy_from_slice(bytes);
            self.index += len;
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}

fn format_error_string<'a>(
    error: impl core::fmt::Debug,
    buffer: &'a mut [u8],
    output: &'a mut [u8],
) -> &'a str {
    let mut writer = SimpleWriter::new(buffer);
    write!(writer, "{:?}", error).unwrap();
    let result = writer.as_str();
    output[..result.len()].copy_from_slice(result.as_bytes());
    core::str::from_utf8(&output[..result.len()]).unwrap()
}
