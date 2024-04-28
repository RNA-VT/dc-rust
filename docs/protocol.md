# Hotline Protocol

This document defines the delimiters and commands for communication between a controller Arduino and digital output Arduino.

## Delimiters

| Name  | Byte 0 | Byte 1 |
|-------|--------|--------|
| Start | 0xBE   | 0xEF   |
| End   | 0xDE   | 0xAD   |

## Commands
### Variables
- $DEVICE_ID:
  - 0xFF (broadcast)
  - 0x00-0xFE (device ID)
- $DIO_ID:
  - 0xFF (broadcast)
  - 0x00 - 0x08 (digital output ID)
- $STATE:
  - 0x00 (low)
  - 0x01 (high)
- $CRC8
  - Calculated CRC

| Name      | Byte 0 | Byte 1 | Byte 2     | Byte 3  | Byte 4 | Byte 5 | Byte 6 | Byte 7 |
|-----------|--------|--------|------------|---------|--------|--------|--------|--------|
| Set State | 0xBE   | 0xEF   | $DEVICE_ID | $DIO_ID | $STATE | $CRC8  | 0xDE   | 0xAD   |

## Example Command Sequences

### Broadcast Blink to all Digital Outputs
loop {
BEEF FF FF FF 00 07 DEAD
delay
BEEF FF FF FF 01 F6 DEAD
delay
}

### Blink to all Digital Outputs on Device 1
loop {
BEEF 01 FF FF 00 7F DEAD
delay
BEEF 01 FF FF 01 8E DEAD
delay
}

### Blink to Digital Output 3 on Device 1
loop {
BEEF 01 03 FF 00 4F DEAD
delay
BEEF 01 03 FF 01 3E DEAD
delay
}
