use std::sync::{Arc, Mutex};

use anyhow::{bail, Result};
use dc::SpecificationEndpoint;
use esp_idf_hal::{
    gpio::{OutputPin, PinDriver},
    peripherals::Peripherals,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::*,
    sys::{
        nvs_flash_erase, nvs_flash_init, ESP_ERR_NVS_NEW_VERSION_FOUND, ESP_ERR_NVS_NO_FREE_PAGES,
        ESP_OK,
    },
};
use log::info;
use wifi::wifi;

use esp_idf_sys as _;

#[toml_cfg::toml_config]
struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_hal::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let sysloop = EspSystemEventLoop::take()?;

    info!("I'm a Relay!");

    info!("Initializing NVS");
    nvs_init().expect("Failed to init NVS");

    info!("Connecting to Wi-Fi...");

    // Load Wi-Fi Config
    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    info!("Load Peripherals / GPIO");
    let peripherals = Peripherals::take()?;

    // Connect to the Wi-Fi network
    let _wifi = match wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => {
            info!("Wi-Fi: Connection Succeeded!");
            inner
        }
        Err(err) => {
            info!("Wi-Fi: Connection Failed.");
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    info!("Get GPIO pin(s)");
    let gpio = peripherals.pins.gpio4.downgrade_output();

    info!("Set Pin IO Mode");
    let mut p = PinDriver::output(gpio)?;

    info!("Set Initial Pin State(s)");
    p.set_high()?;

    info!("Mutex(es)");
    let pin_relay = Arc::new(Mutex::new(p));

    info!("Instantiate Server");
    let mut server = EspHttpServer::new(&Configuration::default())?;

    info!("Adding DC Routes and Handlers");
    server = match dc::server(
        server,
        "relay".to_string(),
        vec![
            SpecificationEndpoint {
                method: String::from("close"),
                parameters: vec![],
            },
            SpecificationEndpoint {
                method: String::from("open"),
                parameters: vec![],
            },
        ],
    ) {
        Ok(server) => {
            info!("DC Server Successfully Started.");
            server
        }
        Err(_e) => {
            info!("!!!ERROR!!!");
            bail!("DC Server Failed to Start.");
        }
    };

    info!("Adding Relay Close Handler");
    server
        .fn_handler("/close", Method::Post, |_request| {
            let mut pin = pin_relay.lock().unwrap();
            match pin.set_low() {
                Ok(()) => Ok(()),
                Err(e) => {
                    info!("Could not set pin state");
                    Err(e)
                }
            }
        })
        .expect("Failed to create /close Handler");

    info!("Adding Relay Open Handler");
    server
        .fn_handler("/open", Method::Post, |_request| {
            let mut pin = pin_relay.lock().unwrap();
            match pin.set_high() {
                Ok(()) => Ok(()),
                Err(e) => {
                    info!("Could not set pin state");
                    Err(e)
                }
            }
        })
        .expect("Failed to create /open Handler");

    loop {
        let mut pin = pin_relay.lock().unwrap();
        if pin.is_set_high() {
            info!("High")
        } else {
            info!("Low")
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn nvs_init() -> Result<(), EspError> {
    unsafe {
        let mut ret = nvs_flash_init();
        if ret == ESP_ERR_NVS_NO_FREE_PAGES || ret == ESP_ERR_NVS_NEW_VERSION_FOUND {
            info!("{}", format_args!("Need to erase flash: rc = {}", ret));
            err(nvs_flash_erase())?;
            ret = nvs_flash_init();
        }
        err(ret)
    }
}

pub fn err(err: i32) -> Result<(), EspError> {
    if err != ESP_OK {
        Err(EspError { code: err })
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct EspError {
    code: i32,
}

impl EspError {
    pub fn code(&self) -> i32 {
        self.code
    }
}

impl From<i32> for EspError {
    fn from(e: i32) -> Self {
        EspError { code: e }
    }
}
