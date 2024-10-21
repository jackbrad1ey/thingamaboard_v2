use crate::{
    key_codes::KeyCode::{self, *},
    NUM_COLS, NUM_ROWS,
};

#[rustfmt::skip]
pub const DEFAULT_LAYER_MAPPING: [[KeyCode; NUM_COLS]; NUM_ROWS] = [
    [Tab, Q, W, F, P, B, CapsLock, Delete, J, L, U, Y, Semicolon, Minus],
    [LeftShift, A, R, S, T, G, LeftWin, Escape, M, N, E, I, O, SingleQuote],
    [Exclamation, Z, X, C, D, V, Empty, Empty, K, H, Comma, Period, ForwardSlash, Empty],
    [Empty, Empty, Empty, LeftAlt, LeftCtrl, LayerShift1, Space, Backspace, Enter, LayerShift2, Empty, Empty, Empty, Empty],
];

#[rustfmt::skip]
pub const NUMPAD_NAV_LAYER_MAPPING: [[KeyCode; NUM_COLS]; NUM_ROWS] = [
    [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Minus, Num1, Num2, Num3, Asterisk, Empty],
    [LeftShift, Left, Up, Down, Right, Empty, Empty, Empty, Plus, Num4, Num5, Num6, Num0, Empty],
    [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Equals, Num7, Num8, Num9, ForwardSlash, Empty],
    [Empty, Empty, Empty, Empty, Empty, Empty, Space, Backspace, Empty, Empty, Empty, Empty, Empty, Empty],
];

#[rustfmt::skip]
pub const SYMBOLS_LAYER_MAPPING: [[KeyCode; NUM_COLS]; NUM_ROWS] = [
    [Empty, Ampersand, At, Caret, Empty, Empty, Empty, Empty, Empty, LeftCurlyBracket, RightCurlyBracket, Empty, Empty, Empty],
    [LeftShift, Tilde, Hash, Asterisk, Equals, Minus, Empty, Empty, Empty, LeftSquareBracket, RightSquareBracket, Empty, Empty, Empty],
    [Empty, BackSlash, Currency, Percent, Plus, Empty, Empty, Empty, Empty, LeftParen, RightParen, Empty, Empty, Empty],
    [Empty, Empty, Empty, Empty, Empty, Empty, Space, Backspace, Empty, Empty, Empty, Empty, Empty, Empty],
];