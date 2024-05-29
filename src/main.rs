#![feature(const_option)]

use esp_idf_svc::hal;
// use esp_idf_sys::tinyusb;
use esp_idf_svc::sys::tinyusb;
use usbd_hid::descriptor::SerializedDescriptor;

use m5atom_bluetooth_keyboard::{usb::Usb, keycode::ToKeyboardReport};


fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let descriptors = &[usbd_hid::descriptor::KeyboardReport::desc()];
    let usb = Usb::new(descriptors);
    usb.init()?;
    log::info!("USB initialized");

    let peripherals = hal::peripherals::Peripherals::take()?;

    let mut button = hal::gpio::PinDriver::input(peripherals.pins.gpio41)?;
    button.set_pull(hal::gpio::Pull::Up)?;
    button.set_interrupt_type(hal::gpio::InterruptType::PosEdge)?;

    let notification = hal::task::notification::Notification::new();
    let notifier = notification.notifier();

    // Safety: make sure the `Notification` object is not dropped while the subscription is active
    unsafe {
        button.subscribe(move || {
            notifier.notify_and_yield(event::BUTTON);
        })?;
    }

    log::info!("Now waiting for a event...");

    loop {
        button.enable_interrupt()?;

        match notification.wait(hal::delay::BLOCK) {
            Some(event::BUTTON) => {
                // input_characters(c"test");
                c"test".input();
            }
            event => println!("Unknown event: {event:?}"),
        }
    }
}

trait InputKeycode {
    fn input(&self);
}

impl InputKeycode for &std::ffi::CStr {
    fn input(&self) {
        for report in self.to_bytes().into_iter().map(|char| (*char).to_report()).flatten() {
            // Press keys
            push_report(&report);

            // Hold key for 10 ms
            hal::delay::FreeRtos::delay_ms(10);

            // Release keys
            push_report(&usbd_hid::descriptor::KeyboardReport::default());
            hal::delay::FreeRtos::delay_ms(10);
        }
    }
}

fn push_report(report: &usbd_hid::descriptor::KeyboardReport) {
    let mut buff: [u8; 64] = [0; 64];
    let size = ssmarshal::serialize(&mut buff, report).unwrap();
    unsafe {
        tinyusb::tud_hid_n_report(
            hid_instance::KEYBOARD,
            0,
            buff.as_ptr() as *const std::ffi::c_void,
            size as u16,
        );
    }
}

pub mod event {
    use std::num::NonZeroU32;

    pub const BUTTON: NonZeroU32 = NonZeroU32::new(1).unwrap();
}

pub mod hid_instance {
    pub const KEYBOARD: u8 = 0;
}
