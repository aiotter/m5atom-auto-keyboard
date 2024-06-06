// references
// https://github.com/esp-rs/esp-idf-sys/issues/301
// https://github.com/esp-rs/esp-idf-hal/issues/231

pub mod descriptor;
pub mod keycode;
pub mod storage;

use esp_idf_svc::sys::{self, tinyusb};

static HID_INSTANCES: once_cell::sync::Lazy<std::sync::Mutex<Vec<HidInstance>>> =
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(vec![]));

pub fn install(hid_instances: &[HidInstance<'static>]) -> anyhow::Result<()> {
    if HID_INSTANCES.lock().unwrap().len() != 0 {
        return Err(anyhow::anyhow!("USB already installed"));
    } else {
        HID_INSTANCES
            .lock()
            .unwrap()
            .extend_from_slice(&hid_instances);
    }

    let mut tusb_cfg: tinyusb::tinyusb_config_t = unsafe { std::mem::zeroed() };
    let config_descriptor = descriptor::config_descriptor(&hid_instances);
    tusb_cfg
        .__bindgen_anon_2
        .__bindgen_anon_1
        .configuration_descriptor = Box::into_raw(config_descriptor) as *const u8;
    // tusb_cfg.__bindgen_anon_1.device_descriptor =
    //     descriptor::string_descriptor_count() as *mut *const i8;

    log::info!("installing USB...");
    sys::esp!(unsafe { tinyusb::tinyusb_driver_install(&tusb_cfg) })?;

    Ok(())
}

pub fn uninstall() -> Result<(), sys::EspError> {
    sys::esp!(unsafe { tinyusb::tinyusb_driver_uninstall() })
}

pub fn is_ready() -> bool {
    unsafe { tinyusb::tud_mounted() }
}

#[derive(Debug, Clone)]
pub struct HidInstance<'a> {
    pub instance_id: u8,
    pub report_id: u8,
    pub descriptor: &'a [u8],
}

impl<'a> HidInstance<'a> {
    pub fn desc(&self) -> &'a [u8] {
        self.descriptor
    }

    // type_keys can only be used for KeyboardReport
    pub fn type_keys<T: keycode::AsKeyboardReport>(&self, keys: &mut dyn Iterator<Item = T>) {
        for report in keys.map(|char| char.as_keyboard_report()).flatten() {
            if report.modifier != 0 {
                let mut modifier_only = report.clone();
                modifier_only.keycodes = [0; 6];
                self.push(&modifier_only);
                esp_idf_svc::hal::delay::FreeRtos::delay_ms(10);
            }

            // Press keys
            self.push(&report);

            // Hold keys for a short period of time
            esp_idf_svc::hal::delay::FreeRtos::delay_ms(50);

            // Release keys
            if report.modifier != 0 {
                let mut modifier_only = report.clone();
                modifier_only.keycodes = [0; 6];
                self.push(&modifier_only);
                esp_idf_svc::hal::delay::FreeRtos::delay_ms(10);
            }
            self.push(&usbd_hid::descriptor::KeyboardReport::default());
            esp_idf_svc::hal::delay::FreeRtos::delay_ms(20);
        }
    }

    pub fn push<T: usbd_hid::descriptor::generator_prelude::Serialize>(&self, report: &T) {
        let mut buff: [u8; 64] = [0; 64];
        let size = ssmarshal::serialize(&mut buff, report).unwrap();
        unsafe {
            tinyusb::tud_hid_n_report(
                self.instance_id,
                self.report_id,
                buff.as_ptr() as *const std::ffi::c_void,
                size as u16,
            );
        }
    }
}

/**  CALLBACKS  **/

// Invoked when received GET HID REPORT DESCRIPTOR
// https://github.com/espressif/esp-idf/blob/4523f2d67465373f0e732a3264273a8e84a1a6d1/examples/peripherals/usb/device/tusb_hid/main/tusb_hid_example_main.c#L62
#[no_mangle]
extern "C" fn tud_hid_descriptor_report_cb(instance: u8) -> *const u8 {
    match HID_INSTANCES
        .lock()
        .unwrap()
        .iter()
        .find(|i| i.instance_id == instance)
    {
        Some(instance) => instance.desc().as_ptr(),
        None => std::ptr::null(),
    }
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
