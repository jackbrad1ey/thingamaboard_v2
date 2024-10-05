use crate::key_codes::KeyCode;
use crate::{
    key_layers,
    NUM_COLS, NUM_ROWS, KEYS_PER_REPORT
};

use embassy_rp::gpio::{Level, Input, Output, Pull};
use embassy_rp::{self, Peripherals};
use embassy_time::Timer;

pub async fn scan_for_keys(keys: &mut [u8; KEYS_PER_REPORT], modifier_byte: &mut u8, columns: &mut [Output<'_>; NUM_COLS], rows: &mut [Input<'_>; NUM_ROWS]) {
    let num_keys = 0;

    for col_num in 0..NUM_COLS {
        columns[col_num].set_high();
        Timer::after_micros(5).await;

        for row_num in 0..NUM_ROWS {
            if rows[row_num].is_high() {
                let keycode = key_layers::DEFAULT_LAYER_MAPPING[row_num][col_num];
                
                if (keycode as u8) & 0xF0 == 0xF0 {  // modifier key
                    *modifier_byte |= 1 << ((keycode as u8) ^ 0xF0)
                } else if num_keys < KEYS_PER_REPORT {
                    keys[num_keys] = keycode as u8;
                }
            }
        }

        columns[col_num].set_low();
    }
}
