use esp_idf_sys::esp_tinyusb::{tinyusb_config_t, tinyusb_driver_install};
use std::time::Duration;
use std::thread;


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let tusb_cfg = tinyusb_config_t::default();

    unsafe { tinyusb_driver_install(&tusb_cfg); }

    log::info!("installed!");

    loop {
        thread::sleep(Duration::from_millis(1000));
        log::info!("sleep...");
    }
}
