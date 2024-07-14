#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_hal::hal::port::{PA0, PA2, PA4, PA6, PC1, PC3, PC4, PC5, PC6, PC7, PE4, PJ0, PJ1};
use arduino_hal::port;
use arduino_hal::port::mode::PullUp;
use arduino_hal::port::mode::{Input, Output};
use arduino_hal::prelude::*;
use max485::Max485;

use hotline::hotline::create_command;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Sign
    let sign_pin_all = pins.d31.into_pull_up_input();
    let sign_pin_input_1 = pins.d22.into_pull_up_input();
    let sign_pin_input_2 = pins.d24.into_pull_up_input();
    let sign_pin_input_3 = pins.d26.into_pull_up_input();
    let sign_pin_input_4 = pins.d28.into_pull_up_input();
    let sign_pin_input_5 = pins.d30.into_pull_up_input();
    let sign_pin_arm = pins.d38.into_pull_up_input();
    let sign_pin_pilot = pins.d40.into_pull_up_input();

    // MegaPoofer
    let mp_pin_all = pins.d33.into_pull_up_input();
    let mp_pin_input_1 = pins.d32.into_pull_up_input();
    let mp_pin_input_2 = pins.d34.into_pull_up_input();
    let mp_pin_input_3 = pins.d36.into_pull_up_input();
    let mp_pin_arm = pins.d42.into_pull_up_input();
    let mp_pin_pilot = pins.d44.into_pull_up_input();

    // Configuration
    let config_pin_enable_sign = pins.d3.into_pull_up_input();
    let config_pin_enable_mp = pins.d4.into_pull_up_input();

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
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(460800), // RS485
    );

    // Max485 initialization
    let mut rs485 = Max485::new(serial, pin_rs485_enable);

    let sign_control_enabled = config_pin_enable_sign.is_high();
    let mp_control_enabled = config_pin_enable_mp.is_high();

    // Initial states for Sign
    let mut previous_sign_state_all = sign_pin_all.is_high();
    let mut previous_sign_states = [
        sign_pin_input_1.is_high(),
        sign_pin_input_2.is_high(),
        sign_pin_input_3.is_high(),
        sign_pin_input_4.is_high(),
        sign_pin_input_5.is_high(),
    ];

    if mp_control_enabled
    
    {
        // Initial states for MegaPoofer
    let mut previous_mp_state_all = mp_pin_all.is_high();
    let mut previous_mp_states = [
        mp_pin_input_1.is_high(),
        mp_pin_input_2.is_high(),
        mp_pin_input_3.is_high(),
    ];
}
    let mut sign_pilot = false;
    let mut mp_pilot = false;
    loop {
        rs485.flush().unwrap();

        if sign_control_enabled {
            // Handle Sign pins
            if sign_pin_arm.is_high() {
                if sign_pin_pilot.is_high() {
                    if !sign_pilot {
                        usb.write_str("Lighting Sign Pilot ...").unwrap();
                        send_command(&mut rs485, 0x00, 0x05, 0x01).unwrap();
                        sign_pilot = true;
                    }
                    check_and_send_sign_commands(
                        &mut rs485,
                        &sign_pin_all,
                        &sign_pin_input_1,
                        &sign_pin_input_2,
                        &sign_pin_input_3,
                        &sign_pin_input_4,
                        &sign_pin_input_5,
                        &mut previous_sign_state_all,
                        &mut previous_sign_states,
                    );
                } else {
                    if sign_pilot {
                        usb.write_str("Turning off Sign Pilot ...").unwrap();
                        send_command(&mut rs485, 0x00, 0x05, 0x00).unwrap();
                        sign_pilot = false;
                    }
                }
            }
        }

        if mp_control_enabled {
            // Handle MegaPoofer pins
            if mp_pin_arm.is_high() {
                if mp_pin_pilot.is_high() {
                    if !mp_pilot {
                        usb.write_str("Lighting MegaPoofer Pilot ...").unwrap();
                        send_command(&mut rs485, 0x01, 0x03, 0x01).unwrap();
                        mp_pilot = true;
                    }
                    check_and_send_mp_commands(
                        &mut rs485,
                        &mp_pin_all,
                        &mp_pin_input_1,
                        &mp_pin_input_2,
                        &mp_pin_input_3,
                        &mut previous_mp_state_all,
                        &mut previous_mp_states,
                    );
                } else {
                    if mp_pilot {
                        usb.write_str("Turning Off MegaPoofer Pilot ...").unwrap();
                        send_command(&mut rs485, 0x01, 0x03, 0x00).unwrap();
                        mp_pilot = false;
                    }
                }
            }
        }
    }
}

fn check_and_send_sign_commands(
    rs485: &mut Max485Type,
    all_pin: &port::Pin<Input<PullUp>, PC6>,
    solenoid_1_pin: &port::Pin<Input<PullUp>, PA0>,
    solenoid_2_pin: &port::Pin<Input<PullUp>, PA2>,
    solenoid_3_pin: &port::Pin<Input<PullUp>, PA4>,
    solenoid_4_pin: &port::Pin<Input<PullUp>, PA6>,
    solenoid_5_pin: &port::Pin<Input<PullUp>, PC7>,
    previous_state_all: &mut bool,
    previous_states: &mut [bool; 5],
) {
    // Check the state of the all pin
    let current_state_all = all_pin.is_high();
    if current_state_all && !*previous_state_all {
        send_command(rs485, 0x00, 0xFF, 0x01).unwrap();
        *previous_state_all = current_state_all;
        return;
    } else if !current_state_all && *previous_state_all {
        send_command(rs485, 0x00, 0xFF, 0x00).unwrap();
        *previous_state_all = current_state_all;
        return;
    }

    let sol1 = solenoid_1_pin.is_high();
    if sol1 != previous_states[1] {
        send_command(rs485, 0x00, 0x00, 0x01).unwrap();
    } else if solenoid_1_pin.is_low() != previous_states[1] {
        send_command(rs485, 0x00, 0x00, 0x00).unwrap();
    }
    previous_states[1] = sol1;

    let sol2 = solenoid_2_pin.is_high();
    if sol2 != previous_states[2] {
        send_command(rs485, 0x00, 0x01, 0x01).unwrap();
    } else if solenoid_2_pin.is_low() != previous_states[2] {
        send_command(rs485, 0x00, 0x01, 0x00).unwrap();
    }
    previous_states[2] = sol2;

    let sol3 = solenoid_3_pin.is_high();
    if sol3 != previous_states[3] {
        send_command(rs485, 0x00, 0x02, 0x01).unwrap();
    } else if solenoid_3_pin.is_low() != previous_states[3] {
        send_command(rs485, 0x00, 0x02, 0x00).unwrap();
    }
    previous_states[3] = sol3;

    let sol4 = solenoid_4_pin.is_high();
    if sol4 != previous_states[4] {
        send_command(rs485, 0x00, 0x03, 0x01).unwrap();
    } else if solenoid_4_pin.is_low() != previous_states[4] {
        send_command(rs485, 0x00, 0x03, 0x00).unwrap();
    }
    previous_states[4] = sol4;

    let sol5 = solenoid_5_pin.is_high();
    if sol5 != previous_states[5] {
        send_command(rs485, 0x00, 0x04, 0x01).unwrap();
    } else if solenoid_5_pin.is_low() != previous_states[5] {
        send_command(rs485, 0x00, 0x04, 0x00).unwrap();
    }
    previous_states[5] = sol5;
}

fn check_and_send_mp_commands(
    rs485: &mut Max485Type,
    all_pin: &port::Pin<Input<PullUp>, PC4>,
    solenoid_1_pin: &port::Pin<Input<PullUp>, PC5>,
    solenoid_2_pin: &port::Pin<Input<PullUp>, PC3>,
    solenoid_3_pin: &port::Pin<Input<PullUp>, PC1>,
    previous_state_all: &mut bool,
    previous_states: &mut [bool; 3],
) {
    // Check the state of the all pin
    let current_state_all = all_pin.is_high();
    if current_state_all && !*previous_state_all {
        send_command(rs485, 0x01, 0xFF, 0x01).unwrap();
        *previous_state_all = current_state_all;
        return;
    } else if !current_state_all && *previous_state_all {
        send_command(rs485, 0x01, 0xFF, 0x00).unwrap();
        *previous_state_all = current_state_all;
        return;
    }

    let sol1 = solenoid_1_pin.is_high();
    if sol1 != previous_states[1] {
        send_command(rs485, 0x01, 0x00, 0x01).unwrap();
    } else if solenoid_1_pin.is_low() != previous_states[1] {
        send_command(rs485, 0x01, 0x00, 0x00).unwrap();
    }
    previous_states[1] = sol1;

    let sol2 = solenoid_2_pin.is_high();
    if sol2 != previous_states[2] {
        send_command(rs485, 0x01, 0x01, 0x01).unwrap();
    } else if solenoid_2_pin.is_low() != previous_states[2] {
        send_command(rs485, 0x01, 0x01, 0x00).unwrap();
    }
    previous_states[2] = sol2;

    let sol3 = solenoid_3_pin.is_high();
    if sol3 != previous_states[3] {
        send_command(rs485, 0x01, 0x02, 0x01).unwrap();
    } else if solenoid_3_pin.is_low() != previous_states[3] {
        send_command(rs485, 0x01, 0x02, 0x00).unwrap();
    }
    previous_states[3] = sol3;
}

type UsartType =
    arduino_hal::Usart<arduino_hal::pac::USART3, port::Pin<Input, PJ0>, port::Pin<Output, PJ1>>;
type Max485Type = Max485<UsartType, port::Pin<Output, PE4>>;

fn send_command(serial: &mut Max485Type, device_id: u8, dio_id: u8, state: u8) -> Result<(), ()> {
    // Create command
    let command = create_command(device_id, dio_id, state);

    // Send command bytes
    for byte in &command {
        match serial.write(*byte) {
            Ok(()) => {}
            Err(_) => return Err(()),
        };
        arduino_hal::delay_us(100);
    }

    Ok(())
}
