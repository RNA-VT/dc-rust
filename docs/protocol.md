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
- $DIO_STATE_0:
  - Device States for Devices 0-7
- $DIO_STATE_1:
  - Device States for Devices 8-15
- $CRC8
  - Calculated CRC


| Name           | Byte 0 | Byte 1 | Byte 2     | Byte 3       | Byte 4       | Byte 5 | Byte 6 | Byte 7 | 
|----------------|--------|--------|------------|--------------|--------------|--------|--------|--------|
| Set Full State | 0xBE   | 0xEF   | $DEVICE_ID | $DIO_STATE_0 | $DIO_1_STATE | $CRC8  | 0xDE   | 0xAD   |
