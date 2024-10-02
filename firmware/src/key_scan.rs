use crate::key_codes::KeyCode;
use crate::{
    key_layers,
    NUM_COLS, NUM_ROWS, KEYS_PER_REPORT
};

use embassy_rp::gpio::{Level, Input, Output, Pull};
use embassy_rp::{self, Peripherals};
use embassy_time::Timer;

fn fill_column_pins(rp: Peripherals, columns: &mut [Output; NUM_COLS]) {
    columns[0] = Output::new(rp.PIN_29,Level::Low);
    columns[1] = Output::new(rp.PIN_28,Level::Low);
    columns[2] = Output::new(rp.PIN_27,Level::Low);
    columns[3] = Output::new(rp.PIN_26,Level::Low);
    columns[4] = Output::new(rp.PIN_25,Level::Low);
    columns[5] = Output::new(rp.PIN_24,Level::Low);
    columns[6] = Output::new(rp.PIN_23,Level::Low);
    columns[7] = Output::new(rp.PIN_6,Level::Low);
    columns[8] = Output::new(rp.PIN_5,Level::Low);
    columns[9] = Output::new(rp.PIN_4,Level::Low);
    columns[10] = Output::new(rp.PIN_3,Level::Low);
    columns[11] = Output::new(rp.PIN_2,Level::Low);
    columns[12] = Output::new(rp.PIN_1,Level::Low);
    columns[13] = Output::new(rp.PIN_0,Level::Low);
}

fn fill_row_pins(rp: Peripherals, rows: &mut [Input; NUM_ROWS]) {
    rows[0] = Input::new(rp.PIN_8, Pull::Down);
    rows[1] = Input::new(rp.PIN_9, Pull::Down);
    rows[2] = Input::new(rp.PIN_10, Pull::Down);
    rows[3] = Input::new(rp.PIN_11, Pull::Down);
}

async fn scan_for_keys(keys: &mut [KeyCode; KEYS_PER_REPORT], modifier_byte: &mut u8, columns: &mut [Output<'_>; NUM_COLS], rows: &mut [Input<'_>; NUM_ROWS]) {
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
                    keys[num_keys] = keycode;
                }
            }
        }

        columns[col_num].set_low();
    }
}
