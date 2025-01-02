use esp_idf_sys::esp_tinyusb::{tinyusb_config_t, tinyusb_driver_install};


fn main() {
    let tusb_cfg = tinyusb_config_t::default();
    unsafe { tinyusb_driver_install(&tusb_cfg); }
}
