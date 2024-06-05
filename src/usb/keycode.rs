use usbd_hid::descriptor::KeyboardReport;
use esp_idf_svc::sys::tinyusb;

pub trait AsKeyboardReport {
    fn as_keyboard_report(self) -> Option<KeyboardReport>;
}

impl AsKeyboardReport for u8 {
    fn as_keyboard_report(self) -> Option<KeyboardReport> {
        character_to_report(self as char)
    }
}

impl AsKeyboardReport for char {
    fn as_keyboard_report(self) -> Option<KeyboardReport> {
        character_to_report(self)
    }
}

#[macro_export]
macro_rules! key {
    // key!(mod(modifier1, modifier2), key1)
    ($(mod($($modifier:expr),*),)? $key:expr) => {
        KeyboardReport {
            modifier: $($(modifier!($modifier) | )*)? 0 as u8,
            reserved: 0,
            leds: 0,
            keycodes: [$key as u8, 0, 0, 0, 0, 0],
        }
    };
}

macro_rules! modifier {
    (ctrl)  => { 0b0001 };
    (shift) => { 0b0010 };
    (alt)   => { 0b0100 };
    (gui)   => { 0b1000 };
    ($modifier:expr) => { $modifier };
}

fn character_to_report(char: char) -> Option<usbd_hid::descriptor::KeyboardReport> {
    // https://github.com/hathach/tinyusb/blob/fd11bf17fde6cbfdb4bb1ed7070ed4111e503ae8/src/class/hid/hid.h#L952-L1099
    use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage::*};

    let shift = tinyusb::hid_keyboard_modifier_bm_t_KEYBOARD_MODIFIER_LEFTSHIFT as u8;

    match char {
        'A'..='Z' => Some(key!(mod(shift), char as u8 - ('A' as u8 - KeyboardAa as u8))),
        'a'..='z' => Some(key!(char as u8 - ('a' as u8 - KeyboardAa as u8))),
        '1'..='9' => Some(key!(char as u8 - ('1' as u8 - Keyboard1Exclamation as u8))),
        '0' => Some(key!(Keyboard0CloseParens)),
        '\x08' => Some(key!(KeyboardBackspace)),
        '\t' => Some(key!(KeyboardTab)),
        '\n' => Some(key!(KeyboardEnter)),
        '\x1b' => Some(key!(KeyboardEscape)),
        '`' => Some(key!(KeyboardBacktickTilde)),
        '~' => Some(key!(mod(shift), KeyboardBacktickTilde)),
        '!' => Some(key!(mod(shift), Keyboard1Exclamation)),
        '@' => Some(key!(mod(shift), Keyboard2At)),
        '#' => Some(key!(mod(shift), Keyboard3Hash)),
        '$' => Some(key!(mod(shift), Keyboard4Dollar)),
        '%' => Some(key!(mod(shift), Keyboard5Percent)),
        '^' => Some(key!(mod(shift), Keyboard6Caret)),
        '&' => Some(key!(mod(shift), Keyboard7Ampersand)),
        '*' => Some(key!(mod(shift), Keyboard8Asterisk)),
        '(' => Some(key!(mod(shift), Keyboard9OpenParens)),
        ')' => Some(key!(mod(shift), Keyboard0CloseParens)),
        '-' => Some(key!(KeyboardDashUnderscore)),
        '_' => Some(key!(mod(shift), KeyboardDashUnderscore)),
        '=' => Some(key!(KeyboardEqualPlus)),
        '+' => Some(key!(mod(shift), KeyboardEqualPlus)),
        '[' => Some(key!(KeyboardOpenBracketBrace)),
        '{' => Some(key!(mod(shift), KeyboardOpenBracketBrace)),
        ']' => Some(key!(KeyboardCloseBracketBrace)),
        '}' => Some(key!(mod(shift), KeyboardCloseBracketBrace)),
        '\\' => Some(key!(KeyboardBackslashBar)),
        '|' => Some(key!(mod(shift), KeyboardBackslashBar)),
        ';' => Some(key!(KeyboardSemiColon)),
        ':' => Some(key!(mod(shift), KeyboardSemiColon)),
        '"' => Some(key!(KeyboardSingleDoubleQuote)),
        '\'' => Some(key!(mod(shift), KeyboardSingleDoubleQuote)),
        ',' => Some(key!(KeyboardCommaLess)),
        '<' => Some(key!(mod(shift), KeyboardCommaLess)),
        '.' => Some(key!(KeyboardPeriodGreater)),
        '/' => Some(key!(KeyboardSlashQuestion)),
        '?' => Some(key!(mod(shift), KeyboardSlashQuestion)),
        ' ' => Some(key!(KeyboardSpacebar)),
        _ => None,
    }
}
