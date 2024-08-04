Hotline Digital Input Controller
========

Controller for MindShark's Sign and MegaPoofer
# 

| Installation | Controlled Device | Control Type     | Control LED High | Control LED GND | Control Input | Control Signal Pin | Hotline Device ID | DIO ID | Output Pin | Relay IN | Relay NO   | Relay COM | Solenoid A | Solenoid B | Notes                                    |
|--------------|-------------------|------------------|------------------|-----------------|---------------|--------------------|-------------------|--------|------------|----------|------------|-----------|------------|------------|------------------------------------------|
|              |                   |                  |                  |                 |               |                    |                   |        |            |          |            |           |            |            |                                          |
| Sign         | Pilot             | Toggle Switch    | --               | GND             | GND           | D12                | 0x00              | 0x00   | D12        | 1        | Solenoid A | +120v     | Relay NO   | -120v      | Pilot solenoid and glowflys              |
| Sign         | All               | Momentary Switch | +5v              | GND             | GND           | D6                 | 0x00              | --     | --         | --       |            |           |            |            | Set all solenoid states                  |
| Sign         | Solenoid 1        | Momentary Switch | +5v              | GND             | GND           | D7                 | 0x00              | 0x01   | D7         | 2        | Solenoid A | +120v     | Relay NO   | -120v      | Far Left                                 |
| Sign         | Solenoid 2        | Momentary Switch | +5v              | GND             | GND           | D8                 | 0x00              | 0x02   | D8         | 3        | Solenoid A | +120v     | Relay NO   | -120v      | Mid Left                                 |
| Sign         | Solenoid 3        | Momentary Switch | +5v              | GND             | GND           | D9                 | 0x00              | 0x03   | D9         | 4        | Solenoid A | +120v     | Relay NO   | -120v      | Center                                   |
| Sign         | Solenoid 4        | Momentary Switch | +5v              | GND             | GND           | D10                | 0x00              | 0x04   | D10        | 5        | Solenoid A | +120v     | Relay NO   | -120v      | Mid Right                                |
| Sign         | Solenoid 5        | Momentary Switch | +5v              | GND             | GND           | D13                | 0x00              | 0x05   | D13        | 6        | Solenoid A | +120v     | Relay NO   | -120v      | Far Right                                |
|              |                   |                  |                  |                 |               |                    |                   |        |            |          |            |           |            |            |                                          |
| MegaPoofer   | Pilot             | Toggle Switch    | --               | GND             | GND           | D12                | 0x01              | 0x00   | D12        | 1        | Solenoid A | +12v      | Relay NO   | GND        | Pilot solenoid and glowflys              |
| MegaPoofer   | All               | Momentary Switch | +5v              | GND             | GND           | D8                 | 0x01              | --     | --         | --       |            |           | Relay NO   | GND        | Set all solenoid states                  |
| MegaPoofer   | Solenoid 1        | Momentary Switch | +5v              | GND             | GND           | D9                 | 0x01              | 0x01   | D7         | 2        | Solenoid A | +12v      | Relay NO   | GND        |                                          |
| MegaPoofer   | Solenoid 2        | Momentary Switch | +5v              | GND             | GND           | D10                | 0x01              | 0x02   | D8         | 3        | Solenoid A | +12v      | Relay NO   | GND        |                                          |
| MegaPoofer   | Solenoid 3        | Momentary Switch | +5v              | GND             | GND           | D13                | 0x01              | 0x03   | D9         | 4        | Solenoid A | +12v      | Relay NO   | GND        |                                          |


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
