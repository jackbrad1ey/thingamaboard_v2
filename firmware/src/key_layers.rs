use crate::{
    key_codes::KeyCode::{self, *},
    NUM_COLS, NUM_ROWS,
};

#[rustfmt::skip]
pub const DEFAULT_LAYER_MAPPING: [[KeyCode; NUM_COLS]; NUM_ROWS] = [
    [Tab, Q, W, F, P, B, Empty, Empty, J, L, U, Y, Semicolon, Empty],
    [LeftShift, A, R, S, T, G, Empty, Empty, M, N, E, I, O, Empty],
    [Empty, Z, X, C, D, V, Empty, Empty, K, H, Comma, Period, ForwardSlash, Empty],
    [Empty, Empty, Empty, Empty, Empty, LayerShift1, Space, Backspace, Enter, Empty, Empty, Empty, Empty, Empty],
];

#[rustfmt::skip]
pub const NUMPAD_NAV_LAYER_MAPPING: [[KeyCode; NUM_COLS]; NUM_ROWS] = [
    [Tab, Q, W, F, P, B, Empty, Empty, J, L, U, Y, Semicolon, Empty],
    [LeftShift, A, R, S, T, G, Empty, Empty, M, N, E, I, O, Empty],
    [Empty, Z, X, C, D, V, Empty, Empty, K, H, Comma, Period, ForwardSlash, Empty],
    [Empty, Empty, Empty, Empty, Empty, LayerShift1, Space, Backspace, LayerShift2, Empty, Empty, Empty, Empty, Empty],
];