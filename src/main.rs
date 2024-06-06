#![feature(const_option)]

use esp_idf_svc::hal;
use smart_leds_trait::SmartLedsWrite;
use std::io::Read as _;
use usbd_hid::descriptor::SerializedDescriptor as _;
use ws2812_esp32_rmt_driver::{lib_smart_leds::Ws2812Esp32Rmt, RGB8};

use m5atom_auto_keyboard::usb;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    usb::storage::init()?;
    usb::storage::mount(c"/usb")?;
    let _ = std::fs::File::create_new("/usb/input.txt").ok();
    // let mut keys: Vec<u8> = std::fs::File::options()
    //     .read(true)
    //     .open("/usb/input.txt")?
    //     .bytes()
    //     .flatten()
    //     .collect();
    usb::storage::unmount()?;

    let keyboard = usb::HidInstance {
        instance_id: 0,
        report_id: 0,
        descriptor: usbd_hid::descriptor::KeyboardReport::desc(),
    };
    let string_descriptor = usb::descriptor::StringDescriptor {
        lang_id: c"\x09\x04",  // English
        manufacturer: c"aiotter",
        product: c"auto-keyboard",
        hid: c"auto-keyboard",
        msc: c"auto-keyboard",
    };
    usb::install(string_descriptor, &[keyboard.clone()])?;

    log::info!("USB initialized");

    let peripherals = hal::peripherals::Peripherals::take()?;

    let mut button = hal::gpio::PinDriver::input(peripherals.pins.gpio41)?;
    button.set_pull(hal::gpio::Pull::UpDown)?;
    button.set_interrupt_type(hal::gpio::InterruptType::AnyEdge)?;

    let notification = hal::task::notification::Notification::new();
    let notifier = notification.notifier();

    let mut led = Ws2812Esp32Rmt::new(peripherals.rmt.channel0, peripherals.pins.gpio35).unwrap();

    // Safety: make sure the `Notification` object is not dropped while the subscription is active
    unsafe {
        button.subscribe(move || {
            notifier.notify_and_yield(event::BUTTON);
        })?;
    }

    log::info!("Now waiting for a event...");

    loop {
        (&mut button).enable_interrupt()?;

        // Show status by LED color
        if usb::storage::is_exposed() {
            #[rustfmt::skip]
            led.write([RGB8 { r: 128, g: 0, b: 0 }].into_iter())?;
        } else {
            #[rustfmt::skip]
            led.write([RGB8 { r: 0, g: 20, b: 50 }].into_iter())?;
        }

        match notification.wait(50) {
            Some(event::BUTTON) => {
                usb::storage::mount(c"/usb")?;
                let keys: Vec<u8> = std::fs::File::options()
                    .read(true)
                    .open("/usb/input.txt")?
                    .bytes()
                    .flatten()
                    .collect();
                usb::storage::unmount()?;
                (&keyboard).type_keys(&mut keys.into_iter());
            }
            event => println!("Unknown event: {event:?}"),
        }
    }
}

pub mod event {
    use std::num::NonZeroU32;

    pub const BUTTON: NonZeroU32 = NonZeroU32::new(1).unwrap();
}
