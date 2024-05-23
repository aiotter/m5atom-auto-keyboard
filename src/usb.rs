// https://github.com/esp-rs/esp-idf-sys/issues/301
// https://github.com/esp-rs/esp-idf-hal/issues/231

use esp_idf_sys::esptinyusb::{HID_KEY_A, tinyusb_config_t, tinyusb_config_t__bindgen_ty_2, tinyusb_config_t__bindgen_ty_2__bindgen_ty_1, tinyusb_driver_install, tud_hid_n_keyboard_report};

fn init() {
    let tusb_cfg = tinyusb_config_t {
        string_descriptor: unsafe { ptr::null_mut() },
        string_descriptor_count: 0,
        external_phy: false,
        __bindgen_anon_1: unsafe { tinyusb_config_t__bindgen_ty_1 { device_descriptor: ptr::null_mut() } },
        __bindgen_anon_2: unsafe { tinyusb_config_t__bindgen_ty_2 { __bindgen_anon_1: tinyusb_config_t__bindgen_ty_2__bindgen_ty_1 { configuration_descriptor: ptr::null_mut() } } },
        self_powered: false,
        vbus_monitor_io: 0,
    };
}
