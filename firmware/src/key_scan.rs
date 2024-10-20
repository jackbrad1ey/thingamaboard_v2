use crate::key_codes::{self, KeyCode};
use crate::{
    key_layers,
    NUM_COLS, NUM_ROWS, KEYS_PER_REPORT
};

use embassy_rp::gpio::{Level, Input, Output, Pull};
use embassy_rp::{self, Peripherals};
use embassy_time::Timer;

pub async fn scan_for_keys(keys: &mut [u8; KEYS_PER_REPORT], modifier_byte: &mut u8, columns: &mut [Output<'_>; NUM_COLS], rows: &mut [Input<'_>; NUM_ROWS]) {
    let mut num_keys = 0;

    let mut layer = key_layers::DEFAULT_LAYER_MAPPING;

    for col_num in 0..NUM_COLS {
        columns[col_num].set_high();
        Timer::after_micros(1).await;

        for row_num in 0..NUM_ROWS {
            if rows[row_num].is_high() {
                let keycode = layer[row_num][col_num];
                
                if keycode == KeyCode::LayerShift1 {  // modifier key
                    layer = key_layers::NUMPAD_NAV_LAYER_MAPPING;
                } else if keycode == KeyCode::LayerShift2 {
                    layer = key_layers::SYMBOLS_LAYER_MAPPING;
                }
            }
        }

        columns[col_num].set_low();
    }

    for col_num in 0..NUM_COLS {
        columns[col_num].set_high();
        Timer::after_micros(1).await;

        for row_num in 0..NUM_ROWS {
            if rows[row_num].is_high() {
                let keycode = layer[row_num][col_num];
                
                if (keycode as u8) & 0xF0 == 0xF0 {  // modifier key
                    *modifier_byte |= 1 << ((keycode as u8) ^ 0xF0)
                } else if num_keys < KEYS_PER_REPORT && !(keycode as u8 - 0xA0 < 16){
                    keys[num_keys] = keycode as u8;
                    num_keys += 1;
                }
            }
        }

        columns[col_num].set_low();
    }
}
