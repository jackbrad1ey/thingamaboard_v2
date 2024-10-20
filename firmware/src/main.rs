#![no_std]
#![no_main]

mod key_codes;
mod key_layers;
mod key_scan;

use defmt::*;
use key_codes::KeyCode;
use core::borrow::BorrowMut;
use core::iter::Empty;
use core::sync::atomic::{AtomicBool, Ordering};

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Input, Output, Pull};
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
use embassy_usb::{Builder, Config, Handler};
use embassy_time::{Duration, Timer};
use embassy_futures::join::join;

use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

use {defmt_rtt as _, panic_probe as _};


const NUM_COLS: usize = 14;
const NUM_ROWS: usize = 4;
const KEYS_PER_REPORT: usize = 6;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let rp = embassy_rp::init(Default::default());
    // Create the driver, from the HAL.
    let driver = Driver::new(rp.USB, Irqs);

    // Create embassy-usb Config
    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Jack Bradley");
    config.product = Some("Thingamaboard V2");
    config.serial_number = Some("000001");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

        // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    // You can also add a Microsoft OS descriptor.
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut request_handler = MyRequestHandler {};
    let mut device_handler = MyDeviceHandler::new();

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    // builder.handler(&mut device_handler);


    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 64,
    };
    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, config);


    // Build and run the USB device
    let mut usb = builder.build();
    let usb_fut = usb.run();

    let (reader, mut writer) = hid.split();

    let mut modifier_byte = 0;
    let mut keycodes: [u8; KEYS_PER_REPORT] = [0; KEYS_PER_REPORT];

    let mut columns: [Output; NUM_COLS] = [
        Output::new(rp.PIN_29,Level::Low),
        Output::new(rp.PIN_28,Level::Low),
        Output::new(rp.PIN_27,Level::Low),
        Output::new(rp.PIN_26,Level::Low),
        Output::new(rp.PIN_25,Level::Low),
        Output::new(rp.PIN_24,Level::Low),
        Output::new(rp.PIN_23,Level::Low),
        Output::new(rp.PIN_6,Level::Low),
        Output::new(rp.PIN_5,Level::Low),
        Output::new(rp.PIN_4,Level::Low),
        Output::new(rp.PIN_3,Level::Low),
        Output::new(rp.PIN_2,Level::Low),
        Output::new(rp.PIN_1,Level::Low),
        Output::new(rp.PIN_0,Level::Low)
    ];

    let mut rows: [Input; NUM_ROWS] = [
        Input::new(rp.PIN_8, Pull::Down),
        Input::new(rp.PIN_9, Pull::Down),
        Input::new(rp.PIN_10, Pull::Down),
        Input::new(rp.PIN_11, Pull::Down)
    ];

    for i in 0..NUM_ROWS {
        rows[i].set_schmitt(true);
    }


    let mut prev_report = KeyboardReport {
        keycodes: keycodes,
        leds: 0,
        modifier: 0,
        reserved: 0
    };

    
    let main_loop = async {
        loop {
            keycodes = [0; KEYS_PER_REPORT];
            modifier_byte = 0;

            key_scan::scan_for_keys(&mut keycodes, &mut modifier_byte, &mut columns, &mut rows).await;
            let report = KeyboardReport {
                keycodes: keycodes,
                leds: 0,
                modifier: modifier_byte,
                reserved: 0,
            };

            if report != prev_report {
                match writer.write_serialize(&report).await {
                    Ok(()) => {}
                    Err(e) => warn!("Failed to send report: {:?}", e),
                };

                prev_report = report;
            }

            Timer::after_micros(1000).await;
        }
    };


    let out_fut = async {
        reader.run(false, &mut request_handler).await;
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join(usb_fut, join(main_loop, out_fut)).await;
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!("Device configured, it may now draw up to the configured current limit from Vbus.")
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
