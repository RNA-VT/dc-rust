#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_hal::hal::port::{
    PE4, PJ0, PJ1,
};
use arduino_hal::port::mode::{Input, Output};
use arduino_hal::prelude::*;
use arduino_hal::port;
use max485::Max485;

use hotline::hotline_protocol::HotlineMessage;
use ufmt::uWrite;

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
    usb.write_str("Serial Initialized").unwrap();

    let mut sign_arm = false;
    let mut sign_pilot = false;
    let sign_enable = config_pin_enable_sign.is_low();
    if sign_enable {
        usb.write_str("[Sign] Enabled\n").unwrap();
    }

    let mut mp_arm = false;
    let mut mp_pilot = false;
    let mp_enable = config_pin_enable_mp.is_low();
    if mp_enable {
        usb.write_str("[MegaPoofer] Enabled\n").unwrap();
    }

    loop {
        rs485.flush().unwrap();
        if sign_enable {
            if sign_pin_arm.is_low() {
                if !sign_arm {
                    usb.write_str("[Sign] Armed...\n").unwrap();
                }
                sign_arm = true;

                let pilot = sign_pin_pilot.is_low();
                if pilot {
                    if !sign_pilot {
                        usb.write_str("[Sign] Lighting Pilot...\n").unwrap();
                    }
                    sign_pilot = true;
                } else {
                    if sign_pilot {
                        usb.write_str("[Sign] Turning Off Pilot...\n").unwrap();
                    }
                }

                let all = sign_pin_all.is_low();

                let mut states: [bool; 16] = [false; 16];
                states[0] = pilot;
                states[1] = sign_pin_input_1.is_low() || all;
                states[2] = sign_pin_input_2.is_low() || all;
                states[3] = sign_pin_input_3.is_low() || all;
                states[4] = sign_pin_input_4.is_low() || all;
                states[5] = sign_pin_input_5.is_low() || all;

                match send_message(&mut rs485, HotlineMessage::new(0x00, states)) {
                    Ok(()) => {}
                    Err(()) => {
                        usb.write_str("[Sign] Error Sending Hotline Message\n")
                            .unwrap();
                    }
                }
            } else {
                if sign_arm {
                    usb.write_str("[Sign] Disarmed...\n").unwrap();
                }
                sign_arm = false;
            }
        }

        if mp_enable {
            if mp_pin_arm.is_low() {
                if !mp_arm {
                    usb.write_str("[MegaPoofer] Armed...\n").unwrap();
                }
                mp_arm = true;

                let pilot = mp_pin_pilot.is_low();
                if pilot {
                    if !mp_pilot {
                        usb.write_str("[MegaPoofer] Lighting Pilot...\n").unwrap();
                    }
                    mp_pilot = true;
                } else {
                    if mp_pilot {
                        usb.write_str("[MegaPoofer] Turning Off Pilot...\n")
                            .unwrap();
                    }
                }

                let all = mp_pin_all.is_low();

                let mut states: [bool; 16] = [false; 16];
                states[0] = pilot;
                states[1] = mp_pin_input_1.is_low() || all;
                states[2] = mp_pin_input_2.is_low() || all;
                states[3] = mp_pin_input_3.is_low() || all;

                let msg = HotlineMessage::new(0x01, states);

                match send_message(&mut rs485, msg) {
                    Ok(()) => {
                        usb.write_str("[MegaPoofer] Successfully Sent Hotline Message\n")
                            .unwrap();
                    }
                    Err(counter) => {
                        usb.write_str("[MegaPoofer] Error Sending Hotline Message\n")
                            .unwrap();
                        ufmt::uwrite!(usb,"Counter: {}\n", counter);
                    }
                }
            } else {
                if mp_arm {
                    usb.write_str("[MegaPoofer] Disarmed...\n").unwrap();
                }
                mp_arm = false;
            }
        }
        arduino_hal::delay_ms(1);
    }
}

type UsartType =
    arduino_hal::Usart<arduino_hal::pac::USART3, port::Pin<Input, PJ0>, port::Pin<Output, PJ1>>;
type Max485Type = Max485<UsartType, port::Pin<Output, PE4>>;

fn send_message(serial: &mut Max485Type, msg: HotlineMessage) -> Result<(), ()> {
    let cmd = msg.to_bytes();
    let mut counter = 0;
    for byte in cmd {
        match serial.write(byte) {
            Ok(()) => {}
            Err(()) => {
                return Err(());
            }
        };
        arduino_hal::delay_us(20);
        counter += 1;
    }
    Ok(())
}
