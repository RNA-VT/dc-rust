Hotline Digital Input Controller
========

Controller for MindShark's Sign and MegaPoofer
# 

| Installation | Controlled Device | Control Type     | Control LED High | Control LED GND | Control Input | Control Signal Pin | Hotline Device ID | DIO ID | Output Pin | Relay IN | Relay NO   | Relay COM | Solenoid A | Solenoid B | Notes                                    |
|--------------|-------------------|------------------|------------------|-----------------|---------------|--------------------|-------------------|--------|------------|----------|------------|-----------|------------|------------|------------------------------------------|
|              |                   |                  |                  |                 |               |                    |                   |        |            |          |            |           |            |            |                                          |
| Sign         | Arm Sign          | Toggle Switch    | --               | GND             | GND           | D38                | --                | --     | --         | --       |            |           |            |            | Enable Sending of Commands to Sign       |
| Sign         | Pilot             | Toggle Switch    | --               | GND             | GND           | D40                | 0x00              | 0x00   | D22        | 1        | Solenoid A | +120v     | Relay NO   | -120v      | Pilot solenoid and glowflys              |
| Sign         | Solenoid 1        | Momentary Switch | +5v              | GND             | GND           | D22                | 0x00              | 0x01   | D24        | 2        | Solenoid A | +120v     | Relay NO   | -120v      | Far Left                                 |
| Sign         | Solenoid 2        | Momentary Switch | +5v              | GND             | GND           | D24                | 0x00              | 0x02   | D26        | 3        | Solenoid A | +120v     | Relay NO   | -120v      | Mid Left                                 |
| Sign         | Solenoid 3        | Momentary Switch | +5v              | GND             | GND           | D26                | 0x00              | 0x03   | D28        | 4        | Solenoid A | +120v     | Relay NO   | -120v      | Center                                   |
| Sign         | Solenoid 4        | Momentary Switch | +5v              | GND             | GND           | D28                | 0x00              | 0x04   | D30        | 5        | Solenoid A | +120v     | Relay NO   | -120v      | Mid Right                                |
| Sign         | Solenoid 5        | Momentary Switch | +5v              | GND             | GND           | D30                | 0x00              | 0x05   | D32        | 6        | Solenoid A | +120v     | Relay NO   | -120v      | Far Right                                |
| Sign         | All               | Momentary Switch | +5v              | GND             | GND           | D31                | 0x00              | --     | --         | --       |            |           |            |            | Set all solenoid states                  |
|              |                   |                  |                  |                 |               |                    |                   |        |            |          |            |           |            |            |                                          |
| MegaPoofer   | Arm Sign          | Toggle Switch    | --               | GND             | GND           | D42                | --                | --     | --         | --       |            |           |            |            | Enable Sending of Commands to MegaPoofer |
| MegaPoofer   | Pilot             | Toggle Switch    | --               | GND             | GND           | D44                | 0x01              | 0x00   | D22        | 1        | Solenoid A | +12v      | Relay NO   | GND        | Pilot solenoid and glowflys              |
| MegaPoofer   | Solenoid 1        | Momentary Switch | +5v              | GND             | GND           | D32                | 0x01              | 0x01   | D24        | 2        | Solenoid A | +12v      | Relay NO   | GND        |                                          |
| MegaPoofer   | Solenoid 2        | Momentary Switch | +5v              | GND             | GND           | D34                | 0x01              | 0x02   | D26        | 3        | Solenoid A | +12v      | Relay NO   | GND        |                                          |
| MegaPoofer   | Solenoid 3        | Momentary Switch | +5v              | GND             | GND           | D36                | 0x01              | 0x03   | D28        | 4        | Solenoid A | +12v      | Relay NO   | GND        |                                          |
| MegaPoofer   | All               | Momentary Switch | +5v              | GND             | GND           | D33                | 0x01              | --     | --         | --       |            |           | Relay NO   | GND        | Set all solenoid states                  |


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
