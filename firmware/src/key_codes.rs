
#[allow(unused)]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum KeyCode {
    Empty = 0x0,

    // Alpha keys
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,

    // Number keys
    Num1 = 0x1E,
    Num2 = 0x1F,
    Num3 = 0x20,
    Num4 = 0x21,
    Num5 = 0x22,
    Num6 = 0x23,
    Num7 = 0x24,
    Num8 = 0x25,
    Num9 = 0x26,
    Num0 = 0x27,

    // Misc keys
    Enter = 0x28,
    Escape = 0x29,
    Backspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Plus = 0x57,
    Equals = 0x2E,
    LeftSquareBracket = 0x2F,
    RightSquareBracket = 0x30,
    BackSlash = 0x31,
    Semicolon = 0x33,
    SingleQuote = 0x34,
    Tilde = 0x35,
    Comma = 0x36,
    Period = 0x37,
    ForwardSlash = 0x38,
    CapsLock = 0x39,
    Asterisk = 0x55,

    Caret = 0xC3,
    Percent = 0xC4,
    Hash = 0xCC,
    At = 0xCE,
    Ampersand = 0xC7,
    Exclamation = 0xCF,
    Currency = 0xB4,


    // Function keys
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,

    // Navigation keys
    Right = 0x4F,
    Left = 0x50,
    Down = 0x51,
    Up = 0x52,
    Home = 0x4A,
    PageUp = 0x4B,
    Delete = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,

    // Media Keys
    VolumeMute = 0x7F,
    VolumeUp = 0x80,
    VolumeDown = 0x81,

    // Keypad keys
    LeftParen = 0xB6,
    RightParen = 0xB7,
    LeftCurlyBracket = 0xB8,
    RightCurlyBracket = 0xB9,

    // Modifier keys
    // Fn = 0xF0,
    LeftShift = 0xF1,
    LeftCtrl = 0xF0,
    LeftAlt = 0xF2,
    LeftCmd = 0xF3,
    RightCmd = 0xF7,
    RightAlt = 0xF6,
    RightCtrl = 0xF4,
    RightShift = 0xF5,
    LeftMeta = 0xF8,

    // Layer change keys
    LayerShift1 = 0xA0,
    LayerShift2 = 0xA1,
    LayerShift3 = 0xA2,
    LayerShift4 = 0xA3,
    LayerShift5 = 0xA4,
    LayerShift6 = 0xA5
}
