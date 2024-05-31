use crate::usb::HidInstance;
use bytes::BufMut;

pub fn string_descriptor() -> [*const std::ffi::CStr; 5] {
    [
        // std::ffi::CString::new([0x09, 0x04]).unwrap().as_c_str(), // 0: is supported language is English (0x0409)
        c"\x09\x04",           // 0: is supported language is English (0x0409)
        c"aiotter",            // 1: Manufacturer
        c"bluetooth-keyboard", // 2: Product
        c"123456",             // 3: Serials, should use chip ID
        c"HID interface",      // 4: HID
    ]
}

pub fn string_descriptor_count() -> usize {
    let mut sum = 0;
    for str in &string_descriptor()[..] {
        unsafe {
            sum = sum + str.as_ref().unwrap().count_bytes();
        }
    }
    sum
}

// https://github.com/espressif/esp-idf/blob/4523f2d67465373f0e732a3264273a8e84a1a6d1/examples/peripherals/usb/device/tusb_hid/main/tusb_hid_example_main.c#L50-L56
#[allow(non_snake_case)]
pub fn config_descriptor(instances: &[HidInstance]) -> Box<[u8]> {
    const BUFFER_SIZE: usize = 128;
    let mut array = [0u8; BUFFER_SIZE];
    let mut buf = &mut array[..];

    // CONFIGURATION DESCRIPTOR
    buf.put_u8(9); // bLength == 9 (const)
    buf.put_u8(2); // bDescriptorType == CONFIGURATION(2) (const)
    buf.put_u16_le(0); // wTotalLength: temporal value
    buf.put_u8(1); // bNumInterface
    buf.put_u8(1); // bConfigurationValue
    buf.put_u8(0); // iConfiguration
    buf.put_u8(0b10100000); // bmAttributes
    buf.put_u8(50); // bMaxPower

    // INTERFACE DESCRIPTOR
    buf.put_u8(9); // bLength == 9 (const)
    buf.put_u8(4); // bDescriptorType == INTERFACE(4) (const)
    buf.put_u8(0); // bInterfaceNumber
    buf.put_u8(0); // bAlternateSetting
    buf.put_u8(1); // bNumEndpoints
    buf.put_u8(3); // bInterfaceClass
    buf.put_u8(0); // bInterfaceSubClass
    buf.put_u8(0); // bInterfaceProtocol
    buf.put_u8(4); // iInterface

    // HID DESCRIPTOR
    buf.put_u8(9); // bLength == 9 (const)
    buf.put_u8(0x21); // bDescriptorType == HID(0x21) (const)
    buf.put_u16_le(0x0111); // bcdHID == v1.11
    buf.put_u8(0); // bCountryCode (0 if not specify)
    buf.put_u8(instances.len() as u8); // bNumDescriptors
    buf.put_u8(0x22); // bDescriptorType (type of HID report descriptor)
    let descriptor_size: u16 = instances.iter().map(|d| d.desc().len() as u16).sum();
    buf.put_u16_le(descriptor_size); // wDescriptorLength

    // ENDPOINT DESCRIPTOR
    buf.put_u8(7); // bLength == 7 (const)
    buf.put_u8(5); // bDescriptorType == ENDPOINT(5) (const)
    buf.put_u8(0b10000001); // bEndpointAddress (bit7: IN=1, OUT=0; bit3-0: Endpoint number);
    buf.put_u8(0b11); // bmAttributes
    buf.put_u16_le(16); // wMaxPacketSize
    buf.put_u8(10); // bInterval

    // Update wTotalLength
    let wTotalLength = BUFFER_SIZE - &buf.remaining_mut();
    let (written, _empty) = array.split_at_mut(wTotalLength);
    written[2..4].copy_from_slice((wTotalLength as u16).to_le_bytes().as_slice());

    Box::from(written.as_ref())
}
