use std::{
    io::{self, ErrorKind},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use hidapi::HidError;

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

        println!("OPeN");
        while shared.run.load(Ordering::SeqCst) {
            let mut buf = [0u8; 64];
            if let Ok(read) = device.read_timeout(&mut buf[..], 16) {
                if read > 0 {
                    println!("Read {read}: {:?}", &buf[..read]);
                }
            }
        }
    } else {
        println!("No Device");
    }

    // Connect to device using its VID and PID
    //let (VID, PID) = (0x0123, 0x3456);
    //let device = api.open(VID, PID).unwrap();

    // Read data from device
    //let mut buf = [0u8; 8];
    //let res = device.read(&mut buf[..]).unwrap();
    //println!("Read: {:?}", &buf[..res]);

    // Write data to device
    //let buf = [0u8, 1, 2, 3, 4];
    //let res = device.write(&buf).unwrap();
    //println!("Wrote: {:?} byte(s)", res);
}
