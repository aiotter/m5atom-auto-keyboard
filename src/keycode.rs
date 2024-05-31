use usbd_hid::descriptor::KeyboardReport;

pub trait AsKeyboardReport {
    fn as_keyboard_report(self) -> Option<KeyboardReport>;
}

impl AsKeyboardReport for u8 {
    fn as_keyboard_report(self) -> Option<KeyboardReport> {
        character_to_report(self as char)
    }
}

fn character_to_report(char: char) -> Option<usbd_hid::descriptor::KeyboardReport> {
    // https://github.com/hathach/tinyusb/blob/fd11bf17fde6cbfdb4bb1ed7070ed4111e503ae8/src/class/hid/hid.h#L952-L1099
    use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage::*};

    macro_rules! key {
        (mod $key:expr) => {
            Some(KeyboardReport {
                modifier: 1,
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

    match char {
        'A'..='Z' => key!(mod char as u8 - ('A' as u8 - KeyboardAa as u8)),
        'a'..='z' => key!(char as u8 - ('a' as u8 - KeyboardAa as u8)),
        '0'..='9' => key!(char as u8 - ('0' as u8 - Keyboard0CloseParens as u8)),
        '\x08' => key!(KeyboardBackspace),
        '\t' => key!(KeyboardTab),
        '\n' => key!(KeyboardEnter),
        '\x1b' => key!(KeyboardEscape),
        '`' => key!(KeyboardBacktickTilde),
        '~' => key!(mod KeyboardBacktickTilde),
        '!' => key!(mod Keyboard1Exclamation),
        '@' => key!(mod Keyboard2At),
        '#' => key!(mod Keyboard3Hash),
        '$' => key!(mod Keyboard4Dollar),
        '%' => key!(mod Keyboard5Percent),
        '^' => key!(mod Keyboard6Caret),
        '&' => key!(mod Keyboard7Ampersand),
        '*' => key!(mod Keyboard8Asterisk),
        '(' => key!(mod Keyboard9OpenParens),
        ')' => key!(mod Keyboard0CloseParens),
        '-' => key!(KeyboardDashUnderscore),
        '_' => key!(mod KeyboardDashUnderscore),
        '=' => key!(KeyboardEqualPlus),
        '+' => key!(mod KeyboardEqualPlus),
        '[' => key!(KeyboardOpenBracketBrace),
        '{' => key!(mod KeyboardOpenBracketBrace),
        ']' => key!(KeyboardCloseBracketBrace),
        '}' => key!(mod KeyboardCloseBracketBrace),
        '\\' => key!(KeyboardBackslashBar),
        '|' => key!(mod KeyboardBackslashBar),
        ';' => key!(KeyboardSemiColon),
        ':' => key!(mod KeyboardSemiColon),
        '"' => key!(KeyboardSingleDoubleQuote),
        '\'' => key!(mod KeyboardSingleDoubleQuote),
        ',' => key!(KeyboardCommaLess),
        '<' => key!(mod KeyboardCommaLess),
        '.' => key!(KeyboardPeriodGreater),
        '/' => key!(KeyboardSlashQuestion),
        '?' => key!(mod KeyboardSlashQuestion),
        ' ' => key!(KeyboardSpacebar),
        _ => None,
    }
}
