#![feature(const_option)]

use esp_idf_svc::hal;
use usbd_hid::descriptor::SerializedDescriptor as _;

use m5atom_bluetooth_keyboard::usb;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let keyboard = usb::HidInstance {
        instance_id: 0,
        report_id: 0,
        descriptor: usbd_hid::descriptor::KeyboardReport::desc(),
    };
    let instances = [keyboard.clone()];
    let usb = usb::Usb::new(&instances[..]);

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
                (&keyboard).type_keys(c"test");
            }
            event => println!("Unknown event: {event:?}"),
        }
    }
}

pub mod event {
    use std::num::NonZeroU32;

    pub const BUTTON: NonZeroU32 = NonZeroU32::new(1).unwrap();
}
