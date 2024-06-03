use usbd_hid::descriptor::KeyboardReport;

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

fn character_to_report(char: char) -> Option<usbd_hid::descriptor::KeyboardReport> {
    // https://github.com/hathach/tinyusb/blob/fd11bf17fde6cbfdb4bb1ed7070ed4111e503ae8/src/class/hid/hid.h#L952-L1099
    use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage::*};

    macro_rules! key {
        (mod($modifier:expr) $key:expr) => {
            Some(KeyboardReport {
                modifier: $modifier as u8,
                reserved: 0,
                leds: 0,
                keycodes: [$key as u8, 0, 0, 0, 0, 0],
            })
        };
        ($key:expr) => {
            Some(KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [$key as u8, 0, 0, 0, 0, 0],
            })
        };
    }

    let shift: u8 = esp_idf_svc::sys::tinyusb::hid_keyboard_modifier_bm_t_KEYBOARD_MODIFIER_LEFTSHIFT.try_into().unwrap();

    match char {
        'A'..='Z' => key!(mod(shift) char as u8 - ('A' as u8 - KeyboardAa as u8)),
        'a'..='z' => key!(char as u8 - ('a' as u8 - KeyboardAa as u8)),
        '1'..='9' => key!(char as u8 - ('1' as u8 - Keyboard1Exclamation as u8)),
        '0' => key!(Keyboard0CloseParens),
        '\x08' => key!(KeyboardBackspace),
        '\t' => key!(KeyboardTab),
        '\n' => key!(KeyboardEnter),
        '\x1b' => key!(KeyboardEscape),
        '`' => key!(KeyboardBacktickTilde),
        '~' => key!(mod(shift) KeyboardBacktickTilde),
        '!' => key!(mod(shift) Keyboard1Exclamation),
        '@' => key!(mod(shift) Keyboard2At),
        '#' => key!(mod(shift) Keyboard3Hash),
        '$' => key!(mod(shift) Keyboard4Dollar),
        '%' => key!(mod(shift) Keyboard5Percent),
        '^' => key!(mod(shift) Keyboard6Caret),
        '&' => key!(mod(shift) Keyboard7Ampersand),
        '*' => key!(mod(shift) Keyboard8Asterisk),
        '(' => key!(mod(shift) Keyboard9OpenParens),
        ')' => key!(mod(shift) Keyboard0CloseParens),
        '-' => key!(KeyboardDashUnderscore),
        '_' => key!(mod(shift) KeyboardDashUnderscore),
        '=' => key!(KeyboardEqualPlus),
        '+' => key!(mod(shift) KeyboardEqualPlus),
        '[' => key!(KeyboardOpenBracketBrace),
        '{' => key!(mod(shift) KeyboardOpenBracketBrace),
        ']' => key!(KeyboardCloseBracketBrace),
        '}' => key!(mod(shift) KeyboardCloseBracketBrace),
        '\\' => key!(KeyboardBackslashBar),
        '|' => key!(mod(shift) KeyboardBackslashBar),
        ';' => key!(KeyboardSemiColon),
        ':' => key!(mod(shift) KeyboardSemiColon),
        '"' => key!(KeyboardSingleDoubleQuote),
        '\'' => key!(mod(shift) KeyboardSingleDoubleQuote),
        ',' => key!(KeyboardCommaLess),
        '<' => key!(mod(shift) KeyboardCommaLess),
        '.' => key!(KeyboardPeriodGreater),
        '/' => key!(KeyboardSlashQuestion),
        '?' => key!(mod(shift) KeyboardSlashQuestion),
        ' ' => key!(KeyboardSpacebar),
        _ => None,
    }
}
