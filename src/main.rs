use esp_idf_svc::{hal, sys};
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

    let peripherals = hal::peripherals::Peripherals::take()?;

    let config = hal::uart::UartConfig::default().baudrate(hal::units::Hertz(115_200));
    let _uart1 = hal::uart::UartDriver::new(
        peripherals.uart1,
        peripherals.pins.gpio42,
        peripherals.pins.gpio40,
        Option::<hal::gpio::AnyIOPin>::None,
        Option::<hal::gpio::AnyIOPin>::None,
        &config,
    )?;
    log::info!("Log is now sent to UART1");

    let mut button = hal::gpio::PinDriver::input(peripherals.pins.gpio41)?;
    button.set_pull(hal::gpio::Pull::Down)?;
    log::info!("Button initialized");

    let mut led = Ws2812Esp32Rmt::new(peripherals.rmt.channel0, peripherals.pins.gpio35).unwrap();
    log::info!("LED initialized");

    // Expose MSC to host machine only when the device is started with its button pressed down
    let is_msc_mode = button.is_low();

    log::info!("MSC mode: {is_msc_mode:?}");

    let keys: Option<Vec<u8>> = if is_msc_mode {
        None
    } else {
        let _mounted = usb::storage::mount_without_msc("/usb")?;
        std::fs::File::create_new("/usb/input.txt").ok();
        let keys: Vec<u8> = std::fs::File::options()
            .read(true)
            .open("/usb/input.txt")?
            .bytes()
            .flatten()
            .collect();

        log::info!(
            "content: {:?}",
            String::from_utf8(keys.clone()).unwrap_or("(cannot print)".into())
        );

        // log::info!("keys: {keys:?}");

        Some(keys)
    };

    let keyboard = usb::HidInstance {
        instance_id: 0,
        report_id: 0,
        descriptor: usbd_hid::descriptor::KeyboardReport::desc(),
    };

    let serial: &'static std::ffi::CStr = {
        let mut id: u64 = 0;
        unsafe { sys::esp_flash_init(core::ptr::null_mut()) };
        sys::esp!(unsafe { sys::esp_flash_read_unique_chip_id(core::ptr::null_mut(), &mut id) })?;
        let boxed = Box::new(std::ffi::CString::new(id.to_string())?);
        Box::leak(boxed)
    };
    let string_descriptor = usb::descriptor::StringDescriptor {
        lang_id: c"\x09\x04", // English
        manufacturer: c"aiotter",
        product: c"auto-keyboard",
        hid: c"auto-keyboard",
        msc: c"auto-keyboard",
        serial: &serial,
    };
    let hid_instances = [keyboard.clone()];
    usb::install(string_descriptor, &hid_instances, is_msc_mode)?;
    log::info!("USB initialized");

    if is_msc_mode {
        usb::storage::init_msc()?;
        std::fs::File::create_new("/usb/input.txt").ok();
    }

    log::info!("Now waiting for a button press...");

    loop {
        // Show status by LED color
        if !is_msc_mode {
            #[rustfmt::skip]
            led.write([RGB8 { r: 0, g: 20, b: 50 }].into_iter())?;
        } else if usb::storage::is_exposed() {
            #[rustfmt::skip]
            led.write([RGB8 { r: 128, g: 0, b: 0 }].into_iter())?;
        } else {
            #[rustfmt::skip]
            led.write([RGB8 { r: 0, g: 0, b: 0 }].into_iter())?;
        }

        if (!is_msc_mode) && button.is_low() {
            // pushed
            while button.is_low() {
                std::thread::sleep(std::time::Duration::from_millis(10))
            }
            if let Some(ref keys) = keys {
                println!("pushing keys: {keys:?}");
                (&keyboard).type_keys(&mut keys.iter().map(|key| key.clone()));
                println!("pushed");
            };
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
