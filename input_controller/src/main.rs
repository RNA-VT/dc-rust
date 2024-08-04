#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;
use arduino_hal::hal::port::{PB4, PB6, PB7, PE4, PH3, PH4, PH5, PH6, PJ0, PJ1, PE0, PE1};
use arduino_hal::port;
use arduino_hal::port::mode::{Input, Output, PullUp};
use arduino_hal::port::Pin;
use arduino_hal::prelude::*;
use max485::Max485;
use arduino_hal::pac::USART0;
use arduino_hal::Usart;

use hotline::hotline_protocol::HotlineMessage;
use ufmt::uWrite;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // RS485 digital output pin
    let mut pin_rs485_enable = pins.d2.into_output();
    pin_rs485_enable.set_high();

    // USB
    let mut usb = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(57600), // USB
    );

    // RS485
    let serial = arduino_hal::Usart::new(
        dp.USART3,
        pins.d15,
        pins.d14.into_output(),
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(460800), // RS485
    );

    // Max485 crate
    let mut rs485 = Max485::new(serial, pin_rs485_enable);
    usb.write_str("Serial Initialized").unwrap();

    // Sign
    let sign_pin_all: Pin<Input<PullUp>, PH3>;
    let sign_pin_input_1: Pin<Input<PullUp>, PH4>;
    let sign_pin_input_2: Pin<Input<PullUp>, PH5>;
    let sign_pin_input_3: Pin<Input<PullUp>, PH6>;
    let sign_pin_input_4: Pin<Input<PullUp>, PB4>;
    let sign_pin_input_5: Pin<Input<PullUp>, PB7>;
    let sign_pin_input_pilot: Pin<Input<PullUp>, PB6>;
    let mut sign_pilot = false;

    // MegaPoofer
    let mp_pin_all: Pin<Input<PullUp>, PH5>;
    let mp_pin_input_1: Pin<Input<PullUp>, PH6>;
    let mp_pin_input_2: Pin<Input<PullUp>, PB4>;
    let mp_pin_input_3: Pin<Input<PullUp>, PB7>;
    let mp_pin_input_pilot: Pin<Input<PullUp>, PB6>;
    let mut mp_pilot = false;

    // Configuration
    let config_pin_enable_sign = pins.d3.into_pull_up_input();
    let config_pin_enable_mp = pins.d4.into_pull_up_input();

    let sign_enable = config_pin_enable_sign.is_low();
    let mp_enable = config_pin_enable_mp.is_low();

    if sign_enable && !mp_enable {
        usb.write_str("[Sign] Enabled\n").unwrap();
        sign_pin_all = pins.d6.into_pull_up_input();
        sign_pin_input_1 = pins.d7.into_pull_up_input();
        sign_pin_input_2 = pins.d8.into_pull_up_input();
        sign_pin_input_3 = pins.d9.into_pull_up_input();
        sign_pin_input_4 = pins.d10.into_pull_up_input();
        sign_pin_input_5 = pins.d13.into_pull_up_input();
        sign_pin_input_pilot = pins.d12.into_pull_up_input();
        loop {
            rs485.flush().unwrap();
            let pilot = sign_pin_input_pilot.is_low();
            if pilot {
                if !sign_pilot {
                    usb.write_str("[Sign] Lighting Pilot...\n").unwrap();
                }
                sign_pilot = true;
            } else if sign_pilot {
                usb.write_str("[Sign] Turning Off Pilot...\n").unwrap();
                sign_pilot = false;
            }

            let all = sign_pin_all.is_low();

            let mut states: [bool; 16] = [false; 16];
            states[0] = pilot;
            states[1] = sign_pin_input_1.is_low() || all;
            states[1] = sign_pin_input_1.is_low() || all;
            states[2] = sign_pin_input_2.is_low() || all;
            states[3] = sign_pin_input_3.is_low() || all;
            states[4] = sign_pin_input_4.is_low() || all;
            states[5] = sign_pin_input_5.is_low() || all;

            match send_message(&mut rs485, HotlineMessage::new(0x00, states), &mut usb) {
                Ok(()) => {}
                Err(()) => {
                    usb.write_str("[Sign] Error Sending Hotline Message\n")
                        .unwrap();
                }
            }
            arduino_hal::delay_ms(10);
        }
    } else if mp_enable && !sign_enable {
        usb.write_str("[MegaPoofer] Enabled\n").unwrap();
        mp_pin_all = pins.d8.into_pull_up_input();
        mp_pin_input_1 = pins.d9.into_pull_up_input();
        mp_pin_input_2 = pins.d10.into_pull_up_input();
        mp_pin_input_3 = pins.d13.into_pull_up_input();
        mp_pin_input_pilot = pins.d12.into_pull_up_input();
        loop {
            rs485.flush().unwrap();
            let pilot = mp_pin_input_pilot.is_low();
            if pilot {
                if !mp_pilot {
                    usb.write_str("[MegaPoofer] Lighting Pilot...\n").unwrap();
                }
                mp_pilot = true;
            } else if mp_pilot {
                usb.write_str("[MegaPoofer] Turning Off Pilot...\n")
                    .unwrap();
                mp_pilot = false;
            }

            let all = mp_pin_all.is_low();

            let mut states: [bool; 16] = [false; 16];
            states[0] = pilot;
            states[1] = mp_pin_input_1.is_low() || all;
            states[2] = mp_pin_input_2.is_low() || all;
            states[3] = mp_pin_input_3.is_low() || all;

            let msg = HotlineMessage::new(0x01, states);

            match send_message(&mut rs485, msg,&mut usb) {
                Ok(()) => {}
                Err(()) => {
                    usb.write_str("[MegaPoofer] Error Sending Hotline Message\n")
                        .unwrap();
                }
            }
            arduino_hal::delay_ms(10);
        }
    } else if sign_enable && mp_enable {
        panic!("This controller cannot function as both an MegaPoofer input and a sign input. please ground only 1 of pin 3 or 4 to select sign or MegaPoofer.");
    } else {
        panic!("Neither Sign nor MegaPoofer is Configured. Please ground pin 3 for the Sign or pin 4 for MegaPoofer.");
    }
}

type UsartType =
    arduino_hal::Usart<arduino_hal::pac::USART3, port::Pin<Input, PJ0>, port::Pin<Output, PJ1>>;
type Max485Type = Max485<UsartType, port::Pin<Output, PE4>>;

fn send_message(serial: &mut Max485Type, msg: HotlineMessage, usb: &mut Usart<USART0, Pin<Input, PE0>, Pin<Output, PE1>>,) -> Result<(), ()> {
    let cmd = msg.to_bytes();
    for byte in cmd {
        match serial.write(byte) {
            Ok(()) => {
                // ufmt::uwrite!(usb, "byte sent: {:X}\n", byte);
            }
            Err(_) => {
                usb.write_str("[RS485] Failed to send byte.");
                return Err(());
            }
        };
        arduino_hal::delay_ms(3);
    }
    Ok(())
}
