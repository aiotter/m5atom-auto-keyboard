// references
// https://github.com/esp-rs/esp-idf-sys/issues/301
// https://github.com/esp-rs/esp-idf-hal/issues/231

use crate::descriptor;
use esp_idf_svc::sys::{self, tinyusb};
use usbd_hid::descriptor::SerializedDescriptor;

type Descriptor = &'static [u8];
static mut INSTALLED_USB: *const Usb = std::ptr::null();

pub struct Usb<'a> {
    descriptors: &'a [Descriptor],
}

impl<'a> Usb<'a> {
    pub fn new(descriptors: &'a [Descriptor]) -> Self {
        Self { descriptors }
    }

    pub fn init(&self) -> anyhow::Result<()> {
        unsafe {
            if INSTALLED_USB != std::ptr::null() {
                panic!("USB already initialized");
            };
            let addr = std::ptr::from_ref(self).addr();
            INSTALLED_USB = std::ptr::without_provenance(addr);
        }

        let mut tusb_cfg: tinyusb::tinyusb_config_t = unsafe { std::mem::zeroed() };
        let config_descriptor = descriptor::config_descriptor(self.descriptors);
        tusb_cfg
            .__bindgen_anon_2
            .__bindgen_anon_1
            .configuration_descriptor = Box::into_raw(config_descriptor) as *const u8;
        // tusb_cfg.__bindgen_anon_1.device_descriptor =
        //     descriptor::string_descriptor_count() as *mut *const i8;

        log::info!("installing USB...");
        match unsafe { tinyusb::tinyusb_driver_install(&tusb_cfg) } {
            sys::ESP_OK => Ok(()),
            err => {
                let err_name = unsafe { std::ffi::CStr::from_ptr(tinyusb::esp_err_to_name(err)) };
                Err(anyhow::anyhow!(
                    "tinyusb_driver_install failed: {err_name:?}"
                ))
            }
        }
        // std::mem::forget(config_descriptor);
    }

    pub fn is_ready(&self) -> bool {
        unsafe { tinyusb::tud_mounted() }
    }
}

impl<'a> Drop for Usb<'a> {
    fn drop(&mut self) {
        if unsafe { tinyusb::tinyusb_driver_uninstall() != sys::ESP_OK } {
            panic!("tinyusb_driver_uninstall failed!");
        }
        unsafe { INSTALLED_USB = std::ptr::null() };
    }
}

// Invoked when received GET HID REPORT DESCRIPTOR
// https://github.com/espressif/esp-idf/blob/4523f2d67465373f0e732a3264273a8e84a1a6d1/examples/peripherals/usb/device/tusb_hid/main/tusb_hid_example_main.c#L62
#[no_mangle]
extern "C" fn tud_hid_descriptor_report_cb(instance: u8) -> *const u8 {
    unsafe { (*INSTALLED_USB).descriptors[instance as usize].as_ptr() }
}

#[no_mangle]
extern "C" fn tud_hid_get_report_cb(
    _instance: u8,
    _report_id: u8,
    _report_type: esp_idf_svc::sys::tinyusb::hid_report_type_t,
    _buffer: *const u8,
    _reqlen: u16,
) -> u16 {
    0
}

#[no_mangle]
extern "C" fn tud_hid_set_report_cb(
    _instance: u8,
    _report_id: u8,
    _report_type: esp_idf_svc::sys::tinyusb::hid_report_type_t,
    _buffer: *const u8,
    _buffsize: u16,
) {
}
