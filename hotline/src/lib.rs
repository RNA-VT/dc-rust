#![no_std]

pub mod hotline {
    use arduino_hal::hal::port::{PE0, PE1};
    use arduino_hal::pac::USART0;
    use arduino_hal::port::mode::{Input, Output};
    use arduino_hal::port::Pin;
    use arduino_hal::Usart;
    use core::fmt::{self, Write};
    use ufmt::uWrite;

    const START_DELIMITER: [u8; 2] = [0xBE, 0xEF];
    const END_DELIMITER: [u8; 2] = [0xDE, 0xAD];

    pub fn calculate_crc(data: &[u8]) -> u8 {
        let mut crc = 0x00;
        for &byte in data {
            crc ^= byte;
            for _ in 0..8 {
                if crc & 0x80 != 0 {
                    crc = (crc << 1) ^ 0x07;
                } else {
                    crc <<= 1;
                }
                crc &= 0xFF; // Ensure CRC remains within 8-bit boundary
            }
        }
        crc
    }

    pub fn create_command(device_id: u8, dio_id: u8, state: u8) -> [u8; 9] {
        let command = [0x00, device_id, dio_id, state];
        let crc = calculate_crc(&command);
        let mut full_command = [0u8; 9];
        full_command[0..2].copy_from_slice(&START_DELIMITER);
        full_command[2..6].copy_from_slice(&command);
        full_command[6] = crc;
        full_command[7..9].copy_from_slice(&END_DELIMITER);
        full_command
    }

    pub fn parse_command(
        buffer: &[u8; 9],
        usb: &mut Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>>,
    ) -> Option<(u8, u8, u8)> {
        let start_delimiter = &buffer[0..2];
        let end_delimiter = &buffer[7..9];

        if start_delimiter != START_DELIMITER || end_delimiter != END_DELIMITER {
            ufmt::uwrite!(usb, "Missing Start or End Delimiter.\r\n").unwrap();
            return None;
        }

        let device_id = buffer[3];
        let dio_id = buffer[4];
        let state = buffer[5];
        let crc = buffer[6];

        let calculated_crc = calculate_crc(&buffer[2..6]);
        if crc != calculated_crc {
            ufmt::uwrite!(usb, "Wrong CRC. Expected: 0x{:02X}\r\n", calculated_crc).unwrap();
            ufmt::uwrite!(usb, "Wrong CRC. Actual: 0x{:02X}\r\n", crc).unwrap();
            ufmt::uwrite!(usb, "Wrong CRC. buffer 0: 0x{:02X}\r\n", buffer[0]).unwrap();
            ufmt::uwrite!(usb, "Wrong CRC. buffer 1: 0x{:02X}\r\n", buffer[1]).unwrap();
            ufmt::uwrite!(usb, "Wrong CRC. buffer 2: 0x{:02X}\r\n", buffer[2]).unwrap();
            ufmt::uwrite!(usb, "Wrong CRC. buffer 3: 0x{:02X}\r\n", buffer[3]).unwrap();
            ufmt::uwrite!(usb, "Wrong CRC. buffer 4: 0x{:02X}\r\n", buffer[4]).unwrap();
            ufmt::uwrite!(usb, "Wrong CRC. buffer 5: 0x{:02X}\r\n", buffer[5]).unwrap();
            ufmt::uwrite!(usb, "Wrong CRC. buffer 6: 0x{:02X}\r\n", buffer[6]).unwrap();
            ufmt::uwrite!(usb, "Wrong CRC. buffer 7: 0x{:02X}\r\n", buffer[7]).unwrap();
            ufmt::uwrite!(usb, "Wrong CRC. buffer 8: 0x{:02X}\r\n", buffer[8]).unwrap();
            return None;
        }

        Some((device_id, dio_id, state))
    }

    struct HexWriter {
        buffer: [u8; 4], // Extended to handle up to 4 characters
        pos: usize,
    }

    impl HexWriter {
        fn new() -> Self {
            HexWriter {
                buffer: [0; 4],
                pos: 0,
            }
        }

        fn as_str(&self) -> &str {
            // Safety: This is safe because we know that `buffer` will always contain valid UTF-8 data.
            core::str::from_utf8(&self.buffer[..self.pos]).unwrap()
        }
    }

    impl fmt::Write for HexWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            for &b in s.as_bytes() {
                if self.pos < self.buffer.len() {
                    self.buffer[self.pos] = b;
                    self.pos += 1;
                }
            }
            Ok(())
        }
    }

    impl ufmt::uWrite for HexWriter {
        type Error = core::convert::Infallible;

        fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
            for &b in s.as_bytes() {
                if self.pos < self.buffer.len() {
                    self.buffer[self.pos] = b;
                    self.pos += 1;
                }
            }
            Ok(())
        }
    }
}
