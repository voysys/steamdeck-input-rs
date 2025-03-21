use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use bytemuck::{from_bytes, from_bytes_mut};
use hidapi::{HidDevice, HidError, HidResult};
use protocol::{
    FeatureReportMsg, SteamDeckStatePacket, ValveInReport,
    FEATURE_REPORT_MESSAGE_ID_CLEAR_DIGITAL_MAPPINGS, HID_FEATURE_REPORT_BYTES,
};

pub mod protocol;

#[derive(Copy, Clone, Default, Debug)]
pub struct GamepadState {
    pub buttons: [u8; 15],
    pub axes: [f32; 6],
}

impl GamepadState {
    fn update(&mut self, new: &SteamDeckStatePacket) {}

    fn fetch(&mut self) -> GamepadState {
        GamepadState::default()
    }
}

struct SteamdeckShared {
    run: AtomicBool,
    found: AtomicBool,
    state: Mutex<GamepadState>,
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
            state: Mutex::new(GamepadState::default()),
        });

        let thread = Some(thread::spawn({
            let shared = shared.clone();
            move || {
                steamdeck_input_thread(shared);
            }
        }));

        SteamdeckInput { shared, thread }
    }

    pub fn state(&self) -> Option<GamepadState> {
        if self.shared.found.load(Ordering::SeqCst) {
            Some(self.shared.state.lock().unwrap().fetch())
        } else {
            None
        }
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

#[derive(Debug)]
pub enum SteamDeckInputError {
    HidError(HidError),
    ProtocolError(String),
}

impl From<HidError> for SteamDeckInputError {
    fn from(hid_error: HidError) -> Self {
        SteamDeckInputError::HidError(hid_error)
    }
}

impl From<String> for SteamDeckInputError {
    fn from(protocol_error: String) -> Self {
        SteamDeckInputError::ProtocolError(protocol_error)
    }
}

const STEAMDECK_VID_PID: (u16, u16) = (0x28de, 0x1205);

fn steamdeck_input_thread(shared: Arc<SteamdeckShared>) {
    'retry: while shared.run.load(Ordering::SeqCst) {
        if let Err(e) = handle_steam_deck_device(&shared) {
            log::error!("SteamDeckError: {e:?}");
        }

        shared.found.store(false, Ordering::SeqCst);
        for _ in 0..100 {
            if !shared.run.load(Ordering::SeqCst) {
                continue 'retry;
            }
            thread::sleep(Duration::from_millis(16));
        }
    }
}

fn handle_steam_deck_device(shared: &SteamdeckShared) -> Result<(), SteamDeckInputError> {
    let api = hidapi::HidApi::new().unwrap();

    let Some(device) = ({
        let mut device = None;

        for device_info in api.device_list() {
            if device_info.vendor_id() == STEAMDECK_VID_PID.0
                && device_info.product_id() == STEAMDECK_VID_PID.1
                && device_info.interface_number() == 2
            {
                device = Some(device_info.open_device(&api)?);
            }
        }
        device
    }) else {
        // Not finding a device is not an error
        return Ok(());
    };

    shared.found.store(true, Ordering::SeqCst);

    disable_deck_lizard_mode(&device)?;

    let mut lizard_counter = 0;

    while shared.run.load(Ordering::SeqCst) {
        let mut buf = [0u8; 64];
        let read = device.read_timeout(&mut buf[..], 16)?;
        if read > 0 {
            let report = from_bytes::<ValveInReport>(&buf[..read]).to_deck_state()?;
            shared.state.lock().unwrap().update(&report);
        } else {
            return Err("Read returned wrong size".to_string().into());
        }

        lizard_counter += 1;
        if lizard_counter > 200 {
            lizard_counter = 0;
            disable_deck_lizard_mode(&device)?;
        }
    }

    Ok(())
}

fn disable_deck_lizard_mode(device: &HidDevice) -> HidResult<()> {
    let mut buf = [0u8; HID_FEATURE_REPORT_BYTES + 1];
    let msg =
        from_bytes_mut::<FeatureReportMsg>(&mut buf[1..(1 + mem::size_of::<FeatureReportMsg>())]);

    msg.header.report_type = FEATURE_REPORT_MESSAGE_ID_CLEAR_DIGITAL_MAPPINGS;

    device.send_feature_report(&buf[..])?;

    Ok(())
}
