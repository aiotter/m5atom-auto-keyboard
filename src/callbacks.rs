use usbd-hid::descriptor::KeyboardReport;

// Invoked when received GET HID REPORT DESCRIPTOR
#[no_mangle]
extern "C" fn tud_hid_descriptor_report_cb(instance: u8) -> *const u8 {
    return KeyboardReport::desc();
}
