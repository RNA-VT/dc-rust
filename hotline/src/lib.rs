#![no_std]

pub mod hotline_protocol {
    const START_DELIMITER: [u8; 2] = [0xBE, 0xEF];
    const END_DELIMITER: [u8; 2] = [0xDE, 0xAD];

    pub struct HotlineMessage {
        pub device_id: u8,
        dio_state_0: u8,
        dio_state_1: u8,
        crc8: u8,
    }

    pub enum ParseError {
        InvalidLength,
        InvalidDelimiters,
        InvalidCrc,
    }

    impl HotlineMessage {
        pub fn new(device_id: u8, states: [bool; 16]) -> Self {
            let (dio_state_0, dio_state_1) = bool_array_to_dio_states(&states);
            let crc8 = calculate_crc8(device_id, dio_state_0, dio_state_1);
            Self {
                device_id,
                dio_state_0,
                dio_state_1,
                crc8,
            }
        }

        pub fn to_bytes(&self) -> [u8; 8] {
            [
                START_DELIMITER[0],
                START_DELIMITER[1],
                self.device_id,
                self.dio_state_0,
                self.dio_state_1,
                self.crc8,
                END_DELIMITER[0],
                END_DELIMITER[1],
            ]
        }

        pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
            if bytes.len() != 8 {
                return Err(ParseError::InvalidLength);
            }

            if &bytes[0..2] != START_DELIMITER || &bytes[6..8] != END_DELIMITER {
                return Err(ParseError::InvalidDelimiters);
            }

            let device_id = bytes[2];
            let dio_state_0 = bytes[3];
            let dio_state_1 = bytes[4];
            let crc8 = bytes[5];

            if crc8 != calculate_crc8(device_id, dio_state_0, dio_state_1) {
                return Err(ParseError::InvalidCrc);
            }

            Ok(Self {
                device_id,
                dio_state_0,
                dio_state_1,
                crc8,
            })
        }

        pub fn get_dio_state(&self, dio_id: usize) -> Option<bool> {
            if dio_id < 8 {
                Some((self.dio_state_0 & (1 << dio_id)) != 0)
            } else if dio_id < 16 {
                Some((self.dio_state_1 & (1 << (dio_id - 8))) != 0)
            } else {
                None
            }
        }
    }

    fn calculate_crc8(device_id: u8, dio_state_0: u8, dio_state_1: u8) -> u8 {
        let mut crc = 0u8;
        let bytes = [device_id, dio_state_0, dio_state_1];

        for &byte in &bytes {
            crc ^= byte;
            for _ in 0..8 {
                if crc & 0x80 != 0 {
                    crc = (crc << 1) ^ 0x07;
                } else {
                    crc <<= 1;
                }
            }
        }
        crc
    }

    pub fn bool_array_to_dio_states(states: &[bool; 16]) -> (u8, u8) {
        let mut dio_state_0 = 0u8;
        let mut dio_state_1 = 0u8;

        for i in 0..8 {
            if states[i] {
                dio_state_0 |= 1 << i;
            }
        }

        for i in 8..16 {
            if states[i] {
                dio_state_1 |= 1 << (i - 8);
            }
        }

        (dio_state_0, dio_state_1)
    }

    pub fn dio_states_to_bool_array(dio_state_0: u8, dio_state_1: u8) -> [bool; 16] {
        let mut states = [false; 16];

        for i in 0..8 {
            states[i] = (dio_state_0 & (1 << i)) != 0;
        }

        for i in 8..16 {
            states[i] = (dio_state_1 & (1 << (i - 8))) != 0;
        }

        states
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_create_message() {
            let states: [bool; 16] = [
                true, false, true, false, true, false, true, false, false, true, false, true,
                false, true, false, true,
            ];
            let message = HotlineMessage::new(0x01, states);
            assert_eq!(message.device_id, 0x01);
            assert_eq!(message.dio_state_0, 0x55); // 0b01010101
            assert_eq!(message.dio_state_1, 0xAA); // 0b10101010
            assert_eq!(message.crc8, calculate_crc8(0x01, 0x55, 0xAA));
        }

        #[test]
        fn test_serialize_message() {
            let states: [bool; 16] = [
                true, false, true, false, true, false, true, false, false, true, false, true,
                false, true, false, true,
            ];
            let message = HotlineMessage::new(0x01, states);
            let bytes = message.to_bytes();
            assert_eq!(
                bytes,
                [0xBE, 0xEF, 0x01, 0x55, 0xAA, message.crc8, 0xDE, 0xAD]
            );
        }

        #[test]
        fn test_parse_message() {
            let states: [bool; 16] = [
                true, false, true, false, true, false, true, false, false, true, false, true,
                false, true, false, true,
            ];
            let original_message = HotlineMessage::new(0x01, states);
            let bytes = original_message.to_bytes();
            let parsed_message = HotlineMessage::from_bytes(&bytes).unwrap();
            assert_eq!(parsed_message, original_message);
        }

        #[test]
        fn test_invalid_message() {
            let invalid_bytes = [0xBE, 0xEF, 0x01, 0xFF, 0x00, 0x00, 0xDE, 0xAD];
            assert!(HotlineMessage::from_bytes(&invalid_bytes).is_none());
        }

        #[test]
        fn test_bool_array_to_dio_states() {
            let states: [bool; 16] = [
                true, false, true, false, true, false, true, false, false, true, false, true,
                false, true, false, true,
            ];

            let (dio_state_0, dio_state_1) = bool_array_to_dio_states(&states);
            assert_eq!(dio_state_0, 0b01010101);
            assert_eq!(dio_state_1, 0b10101010);
        }

        #[test]
        fn test_dio_states_to_bool_array() {
            let dio_state_0 = 0b01010101;
            let dio_state_1 = 0b10101010;
            let expected_states: [bool; 16] = [
                true, false, true, false, true, false, true, false, false, true, false, true,
                false, true, false, true,
            ];

            let states = dio_states_to_bool_array(dio_state_0, dio_state_1);
            assert_eq!(states, expected_states);
        }

        #[test]
        fn test_all_false() {
            let states: [bool; 16] = [false; 16];
            let (dio_state_0, dio_state_1) = bool_array_to_dio_states(&states);
            assert_eq!(dio_state_0, 0b00000000);
            assert_eq!(dio_state_1, 0b00000000);

            let bool_array = dio_states_to_bool_array(dio_state_0, dio_state_1);
            assert_eq!(bool_array, states);
        }

        #[test]
        fn test_all_true() {
            let states: [bool; 16] = [true; 16];
            let (dio_state_0, dio_state_1) = bool_array_to_dio_states(&states);
            assert_eq!(dio_state_0, 0b11111111);
            assert_eq!(dio_state_1, 0b11111111);

            let bool_array = dio_states_to_bool_array(dio_state_0, dio_state_1);
            assert_eq!(bool_array, states);
        }

        #[test]
        fn test_get_dio_state() {
            let states: [bool; 16] = [
                true, false, true, false, true, false, true, false, false, true, false, true,
                false, true, false, true,
            ];
            let message = HotlineMessage::new(0x01, states);
            assert_eq!(message.get_dio_state(0), Some(true));
            assert_eq!(message.get_dio_state(1), Some(false));
            assert_eq!(message.get_dio_state(8), Some(false));
            assert_eq!(message.get_dio_state(9), Some(true));
            assert_eq!(message.get_dio_state(15), Some(true));
            assert_eq!(message.get_dio_state(16), None); // Out of range
        }
    }
}
