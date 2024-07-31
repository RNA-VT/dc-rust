Hotline Digital Input Controller
========

Controller for MindShark's Sign and MegaPoofer
# 

| Installation | Controlled Device | Control          | Input Pin | Hotline Device ID | DIO ID | Output Pin | Relay Input/Output | Notes                                    |
|--------------|-------------------|------------------|-----------|-------------------|--------|------------|--------------------|------------------------------------------|
|              |                   |                  |           |                   |        |            |                    |                                          |
| Sign         | Arm Sign          | Covered Switch   | D38       | --                | --     | --         | --                 | Enable Sending of Commands to Sign       |
| Sign         | Pilot             | Covered Switch   | D40       | 0x00              | 0x05   | D22        | 1                  | Pilot solenoid and glowflys              |
| Sign         | Solenoid 1        | Momentary Switch | D22       | 0x00              | 0x00   | D24        | 2                  |                                          |
| Sign         | Solenoid 2        | Momentary Switch | D24       | 0x00              | 0x01   | D26        | 3                  |                                          |
| Sign         | Solenoid 3        | Momentary Switch | D26       | 0x00              | 0x02   | D28        | 4                  |                                          |
| Sign         | Solenoid 4        | Momentary Switch | D28       | 0x00              | 0x03   | D30        | 5                  |                                          |
| Sign         | Solenoid 5        | Momentary Switch | D30       | 0x00              | 0x04   | D32        | 6                  |                                          |
| Sign         | All               | Momentary Switch | D31       | 0x00              | 0xFF   | --         | --                 | Set all solenoid states                  |
|              |                   |                  |           |                   |        |            |                    |                                          |
| MegaPoofer   | Arm Sign          | Covered Switch   | D42       | --                | --     | --         | --                 | Enable Sending of Commands to MegaPoofer |
| MegaPoofer   | Pilot             | Covered Switch   | D44       | 0x01              | 0x00   | D22        | 1                  | Pilot solenoid and glowflys              |
| MegaPoofer   | Solenoid 1        | Momentary Switch | D32       | 0x01              | 0x01   | D24        | 2                  |                                          |
| MegaPoofer   | Solenoid 2        | Momentary Switch | D34       | 0x01              | 0x02   | D26        | 3                  |                                          |
| MegaPoofer   | Solenoid 3        | Momentary Switch | D36       | 0x01              | 0x03   | D28        | 4                  |                                          |
| MegaPoofer   | All               | Momentary Switch | D33       | 0x01              | 0xFF   | --         | --                 | Set all solenoid states                  |


## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under:
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
