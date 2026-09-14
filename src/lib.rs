use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use bytemuck::from_bytes_mut;
use hidapi::{HidDevice, HidError, HidResult};
use protocol::{
    DigitalMapping, FeatureReportMsg, SteamDeckStatePacket, ValveInReport, BUTTON_A, BUTTON_B,
    BUTTON_DPAD_DOWN, BUTTON_DPAD_LEFT, BUTTON_DPAD_RIGHT, BUTTON_DPAD_UP, BUTTON_L4, BUTTON_L5,
    BUTTON_LEFT_BUMPER, BUTTON_LEFT_PAD, BUTTON_LEFT_STICK, BUTTON_MENU, BUTTON_QUICK_ACCESS,
    BUTTON_R4, BUTTON_R5, BUTTON_RIGHT_BUMPER, BUTTON_RIGHT_PAD, BUTTON_RIGHT_STICK, BUTTON_STEAM,
    BUTTON_VIEW, BUTTON_X, BUTTON_Y, FEATURE_REPORT_MESSAGE_ID_CLEAR_DIGITAL_MAPPINGS,
    FEATURE_REPORT_MESSAGE_ID_SET_DIGITAL_MAPPINGS, HID_FEATURE_REPORT_BYTES,
};

pub mod protocol;

#[derive(Copy, Clone, Default, Debug)]
pub struct GamepadState {
    pub buttons: [u8; 22],
    pub axes: [f32; 6],
}

#[derive(Copy, Clone, Debug)]
pub struct GamepadUpdateState {
    pub gamepad: GamepadState,
    pub last_update_time: Instant,
    pub fetched: bool,
}

impl GamepadUpdateState {
    fn update(&mut self, new: &SteamDeckStatePacket) {
        self.gamepad.axes[0] = (new.left_stick_x as f32 / i16::MAX as f32).clamp(-1.0, 1.0);
        self.gamepad.axes[1] = -(new.left_stick_y as f32 / i16::MAX as f32).clamp(-1.0, 1.0);
        self.gamepad.axes[2] = (new.right_stick_x as f32 / i16::MAX as f32).clamp(-1.0, 1.0);
        self.gamepad.axes[3] = -(new.right_stick_y as f32 / i16::MAX as f32).clamp(-1.0, 1.0);
        self.gamepad.axes[4] =
            (new.trigger_raw_l as f32 / i16::MAX as f32).clamp(0.0, 1.0) * 2.0 - 1.0;
        self.gamepad.axes[5] =
            (new.trigger_raw_r as f32 / i16::MAX as f32).clamp(0.0, 1.0) * 2.0 - 1.0;

        let b = &mut self.gamepad.buttons;

        b[0] = (((new.buttons & BUTTON_A) > 0) || (b[0] != 0 && !self.fetched)) as u8;
        b[1] = (((new.buttons & BUTTON_B) > 0) || (b[1] != 0 && !self.fetched)) as u8;
        b[2] = (((new.buttons & BUTTON_X) > 0) || (b[2] != 0 && !self.fetched)) as u8;
        b[3] = (((new.buttons & BUTTON_Y) > 0) || (b[3] != 0 && !self.fetched)) as u8;
        b[4] = (((new.buttons & BUTTON_LEFT_BUMPER) > 0) || (b[4] != 0 && !self.fetched)) as u8;
        b[5] = (((new.buttons & BUTTON_RIGHT_BUMPER) > 0) || (b[5] != 0 && !self.fetched)) as u8;
        b[6] = (((new.buttons & BUTTON_VIEW) > 0) || (b[6] != 0 && !self.fetched)) as u8;
        b[7] = (((new.buttons & BUTTON_MENU) > 0) || (b[7] != 0 && !self.fetched)) as u8;
        b[8] = (((new.buttons & BUTTON_QUICK_ACCESS) > 0) || (b[8] != 0 && !self.fetched)) as u8;
        b[9] = (((new.buttons & BUTTON_LEFT_STICK) > 0) || (b[9] != 0 && !self.fetched)) as u8;
        b[10] = (((new.buttons & BUTTON_RIGHT_STICK) > 0) || (b[10] != 0 && !self.fetched)) as u8;
        b[11] = (((new.buttons & BUTTON_DPAD_UP) > 0) || (b[11] != 0 && !self.fetched)) as u8;
        b[12] = (((new.buttons & BUTTON_DPAD_RIGHT) > 0) || (b[12] != 0 && !self.fetched)) as u8;
        b[13] = (((new.buttons & BUTTON_DPAD_DOWN) > 0) || (b[13] != 0 && !self.fetched)) as u8;
        b[14] = (((new.buttons & BUTTON_DPAD_LEFT) > 0) || (b[14] != 0 && !self.fetched)) as u8;
        b[15] = (((new.buttons & BUTTON_R4) > 0) || (b[15] != 0 && !self.fetched)) as u8;
        b[16] = (((new.buttons & BUTTON_R5) > 0) || (b[16] != 0 && !self.fetched)) as u8;
        b[17] = (((new.buttons & BUTTON_L4) > 0) || (b[17] != 0 && !self.fetched)) as u8;
        b[18] = (((new.buttons & BUTTON_L5) > 0) || (b[18] != 0 && !self.fetched)) as u8;
        b[19] = (((new.buttons & BUTTON_STEAM) > 0) || (b[19] != 0 && !self.fetched)) as u8;
        b[20] = (((new.buttons & BUTTON_LEFT_PAD) > 0) || (b[20] != 0 && !self.fetched)) as u8;
        b[21] = (((new.buttons & BUTTON_RIGHT_PAD) > 0) || (b[21] != 0 && !self.fetched)) as u8;

        self.last_update_time = Instant::now();
        self.fetched = false;
    }

    fn fetch(&mut self) -> Option<GamepadState> {
        self.fetched = true;
        if self.last_update_time.elapsed() < Duration::from_millis(100) {
            Some(self.gamepad)
        } else {
            None
        }
    }
}

struct SteamdeckShared {
    run: AtomicBool,
    state: Mutex<Option<GamepadUpdateState>>,
}

pub struct SteamdeckInput {
    shared: Arc<SteamdeckShared>,
    thread: Option<JoinHandle<()>>,
}

impl SteamdeckInput {
    pub fn new() -> SteamdeckInput {
        Self::try_new().expect("failed to start Steam Deck input worker")
    }

    /// Starts one owned worker. Dropping the reader stops and joins it.
    pub fn try_new() -> std::io::Result<SteamdeckInput> {
        let shared = Arc::new(SteamdeckShared {
            run: AtomicBool::new(true),
            state: Mutex::new(None),
        });
        let worker_shared = Arc::clone(&shared);
        let thread = thread::Builder::new()
            .name("Steam Deck input".into())
            .spawn(move || steamdeck_input_thread(worker_shared))?;
        Ok(SteamdeckInput {
            shared,
            thread: Some(thread),
        })
    }

    pub fn fetch(&self) -> Option<GamepadState> {
        self.read_state(true)
    }

    /// Inspect availability/state without consuming retained short presses.
    pub fn peek(&self) -> Option<GamepadState> {
        self.read_state(false)
    }

    fn read_state(&self, consume: bool) -> Option<GamepadState> {
        let mut state = self.shared.state.lock().unwrap();
        let result = state.as_mut().and_then(|state| {
            if consume {
                state.fetch()
            } else if state.last_update_time.elapsed() < Duration::from_millis(100) {
                Some(state.gamepad)
            } else {
                None
            }
        });
        if result.is_none() {
            *state = None;
        }
        result
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
            thread.thread().unpark();
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
    let mut last_error = None;
    let mut retry_delay = Duration::from_millis(100);
    while shared.run.load(Ordering::Relaxed) {
        let result = handle_steam_deck_device(&shared);
        *shared.state.lock().unwrap() = None;
        let error = result.err().map(|e| format!("{e:?}"));
        if error != last_error {
            if let Some(error) = &error {
                log::warn!("Steam Deck input unavailable: {error}");
            }
            last_error = error;
        }
        // Unpark on shutdown, including when no device is present.
        if shared.run.load(Ordering::Relaxed) {
            thread::park_timeout(retry_delay);
            retry_delay = (retry_delay * 2).min(Duration::from_secs(2));
        }
    }
}

fn handle_steam_deck_device(shared: &SteamdeckShared) -> Result<(), SteamDeckInputError> {
    let api = hidapi::HidApi::new()?;
    let mut candidates = std::collections::BTreeMap::new();
    for info in api.device_list() {
        if (info.vendor_id(), info.product_id()) == STEAMDECK_VID_PID
            && info.interface_number() == 2
            && info.usage_page() == 0xffff
            && info.usage() == 1
        {
            candidates.entry(info.path().to_owned()).or_insert(info);
        }
    }
    if candidates.is_empty() {
        return Ok(());
    }
    if candidates.len() != 1 {
        return Err(
            "multiple Steam Deck gamepad paths; refusing ambiguous ownership"
                .to_string()
                .into(),
        );
    }
    let (path, info) = candidates.into_iter().next().unwrap();
    let device = info
        .open_device(&api)
        .map_err(|e| format!("open {}: {e}", path.to_string_lossy()))?;
    log::debug!("Steam Deck hidraw opened {}", path.to_string_lossy());
    let result = read_device(shared, &device);
    *shared.state.lock().unwrap() = None;
    drop(device);
    log::debug!("Steam Deck hidraw closed {}", path.to_string_lossy());
    result.map_err(|e| format!("read/configure {}: {e:?}", path.to_string_lossy()).into())
}

trait Controller {
    fn read(&self, bytes: &mut [u8]) -> HidResult<usize>;
    fn configure(&self) -> HidResult<()>;
}

impl Controller for HidDevice {
    fn read(&self, bytes: &mut [u8]) -> HidResult<usize> {
        self.read_timeout(bytes, 16)
    }
    fn configure(&self) -> HidResult<()> {
        disable_deck_lizard_mode(self)
    }
}

struct ClearStateOnClose<'a>(&'a SteamdeckShared);

impl Drop for ClearStateOnClose<'_> {
    fn drop(&mut self) {
        *self.0.state.lock().unwrap() = None;
    }
}

fn read_device(
    shared: &SteamdeckShared,
    device: &impl Controller,
) -> Result<(), SteamDeckInputError> {
    let _clear_on_close = ClearStateOnClose(shared);
    let mut configured = false;
    let mut warned_invalid = false;
    while shared.run.load(Ordering::Relaxed) {
        let mut bytes = [0; 256];
        let count = match device.read(&mut bytes) {
            Ok(count) => count,
            Err(_) if !shared.run.load(Ordering::Relaxed) => break,
            Err(error) => return Err(error.into()),
        };
        if count == 0 {
            // No new data: keep ownership, but never expose stale input.
            let mut state = shared.state.lock().unwrap();
            if state
                .as_ref()
                .is_some_and(|s| s.last_update_time.elapsed() >= Duration::from_millis(100))
            {
                *state = None;
            }
            continue;
        }
        let report = match ValveInReport::parse_deck_state(&bytes[..count]) {
            Ok(Some(report)) => report,
            Ok(None) => continue,
            Err(error) => {
                if !warned_invalid {
                    log::warn!("Ignoring malformed Steam Deck input: {error}");
                    warned_invalid = true;
                }
                continue;
            }
        };
        if !configured {
            // Confirm the intended interface with an actual state before writes.
            // The isolated experiment verified this sequence without periodic
            // reapplication. Do not race other clients by replaying it on reads.
            device.configure()?;
            configured = true;
            log::info!("Steam Deck input available via hidraw");
            continue; // Publish a fresh report received after configuration.
        }
        let mut state = shared.state.lock().unwrap();
        if state
            .as_ref()
            .is_some_and(|s| s.last_update_time.elapsed() >= Duration::from_millis(100))
        {
            *state = None;
        }
        let state = state.get_or_insert(GamepadUpdateState {
            gamepad: GamepadState::default(),
            last_update_time: Instant::now(),
            fetched: true,
        });
        state.update(&report);
    }
    Ok(())
}

fn disable_deck_lizard_mode(device: &HidDevice) -> HidResult<()> {
    {
        let mut buf = [0u8; HID_FEATURE_REPORT_BYTES + 1];
        let msg = from_bytes_mut::<FeatureReportMsg>(
            &mut buf[1..(1 + mem::size_of::<FeatureReportMsg>())],
        );

        msg.header.report_type = FEATURE_REPORT_MESSAGE_ID_CLEAR_DIGITAL_MAPPINGS;
        device.send_feature_report(&buf[..])?;
    }

    {
        let mut buf = [0u8; HID_FEATURE_REPORT_BYTES + 1];
        let msg = from_bytes_mut::<FeatureReportMsg>(
            &mut buf[1..(1 + mem::size_of::<FeatureReportMsg>())],
        );

        msg.header.report_type = FEATURE_REPORT_MESSAGE_ID_SET_DIGITAL_MAPPINGS;
        msg.header.report_length = (2 * mem::size_of::<DigitalMapping>()) as u8;
        unsafe {
            msg.payload.set_digital_mappings.mappings[0].buttons = BUTTON_RIGHT_PAD;
            msg.payload.set_digital_mappings.mappings[0].emulated_device_type = 1;
            msg.payload.set_digital_mappings.mappings[0].emulated_button = 1;
            msg.payload.set_digital_mappings.mappings[1].buttons = BUTTON_LEFT_PAD;
            msg.payload.set_digital_mappings.mappings[1].emulated_device_type = 1;
            msg.payload.set_digital_mappings.mappings[1].emulated_button = 2;
        }
        device.send_feature_report(&buf[..])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests;
