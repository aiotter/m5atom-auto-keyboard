use esp_idf_sys::{esptinyusb, tinyusb};

fn init() {
    let tusb_cfg = esptinyusb::tinyusb_config_t {
        string_descriptor: ptr::null_mut(),
        string_descriptor_count: 0,
        external_phy: false,
        __bindgen_anon_1: unsafe {
            esptinyusb::tinyusb_config_t__bindgen_ty_1 {
                device_descriptor: std::ptr::null_mut(),
            }
        },
        __bindgen_anon_2: unsafe {
            esptinyusb::tinyusb_config_t__bindgen_ty_2 {
                __bindgen_anon_1: esptinyusb::tinyusb_config_t__bindgen_ty_2__bindgen_ty_1 {
                    configuration_descriptor: CONFIG_DESC.as_ptr(),
                },
            }
        },
        self_powered: false,
        vbus_monitor_io: 0,
    };

    unsafe { esptinyusb::tinyusb_driver_install(&tusb_cfg); }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
}
