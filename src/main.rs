use esp_idf_sys::{tinyusb_config_t, tinyusb_driver_install, tinyusb_msc_storage_mount};
use std::ffi::CString;

fn main() {
    let tusb_cfg = tinyusb_config_t::default();
    unsafe {
        tinyusb_driver_install(&tusb_cfg);
    }

    let s = CString::new("");
    unsafe { tinyusb_msc_storage_mount(s.unwrap().as_ptr()) };
}
