use std::{
    io::{self, ErrorKind},
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use bytemuck::{from_bytes, from_bytes_mut};
use hidapi::{HidDevice, HidError, HidResult};
use protocol::{
    FeatureReportMsg, ValveInReport, FEATURE_REPORT_MESSAGE_ID_CLEAR_DIGITAL_MAPPINGS,
    HID_FEATURE_REPORT_BYTES,
};

pub mod protocol;

struct SteamdeckShared {
    run: AtomicBool,
    found: AtomicBool,
}

pub struct SteamdeckInput {
    shared: Arc<SteamdeckShared>,
    thread: Option<JoinHandle<()>>,
}

impl SteamdeckInput {
    pub fn new() -> SteamdeckInput {
        let shared = Arc::new(SteamdeckShared {
            found: AtomicBool::new(false),
            run: AtomicBool::new(true),
        });

        let thread = Some(thread::spawn({
            let shared = shared.clone();
            move || {
                steamdeck_input_thread(shared);
            }
        }));

        SteamdeckInput { shared, thread }
    }
}

impl Default for SteamdeckInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SteamdeckInput {
    fn drop(&mut self) {
        self.shared.run.store(false, Ordering::SeqCst);
        if let Some(thread) = self.thread.take() {
            thread.join().ok();
        }
    }
}

const STEAMDECK_VID_PID: (u16, u16) = (0x28de, 0x1205);

fn steamdeck_input_thread(shared: Arc<SteamdeckShared>) {
    let api = hidapi::HidApi::new().unwrap();
    // Print out information about all connected devices

    let device = {
        let mut device = Err(HidError::IoError {
            error: io::Error::new(ErrorKind::NotFound, "No steamdeck found"),
        });

        for device_info in api.device_list() {
            if device_info.vendor_id() == STEAMDECK_VID_PID.0
                && device_info.product_id() == STEAMDECK_VID_PID.1
                && device_info.interface_number() == 2
            {
                device = device_info.open_device(&api);
            }
        }
        device
    };

    if let Ok(device) = device.as_ref() {
        shared.found.store(true, Ordering::SeqCst);

        if let Err(e) = disable_deck_lizard_mode(device) {
            println!("Failed to disable lizard mode: {e:?}");
        }

        let mut lizard_counter = 0;

        while shared.run.load(Ordering::SeqCst) {
            let mut buf = [0u8; 64];
            if let Ok(read) = device.read_timeout(&mut buf[..], 16) {
                if read > 0 {
                    let report = from_bytes::<ValveInReport>(&buf[..read]).to_deck_state();

                    match report {
                        Ok(report) => {
                            println!("Read {read}: {:#?}", report);
                        }
                        Err(err) => {
                            println!("Error {read}: {:#?}", err);
                        }
                    }
                }
            }

            lizard_counter += 1;
            if lizard_counter > 200 {
                lizard_counter = 0;
                if let Err(e) = disable_deck_lizard_mode(device) {
                    println!("Failed to disable lizard mode: {e:?}");
                }
            }
        }
    } else {
        println!("No Device");
    }
}

fn disable_deck_lizard_mode(device: &HidDevice) -> HidResult<()> {
    let mut buf = [0u8; HID_FEATURE_REPORT_BYTES + 1];
    let msg =
        from_bytes_mut::<FeatureReportMsg>(&mut buf[1..(1 + mem::size_of::<FeatureReportMsg>())]);

    msg.header.report_type = FEATURE_REPORT_MESSAGE_ID_CLEAR_DIGITAL_MAPPINGS;

    device.send_feature_report(&buf[..])?;

    Ok(())
}
