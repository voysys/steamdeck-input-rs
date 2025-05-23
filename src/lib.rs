use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use bytemuck::{from_bytes, from_bytes_mut};
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
    found: AtomicBool,
    state: Mutex<GamepadUpdateState>,
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
            state: Mutex::new(GamepadUpdateState {
                gamepad: Default::default(),
                last_update_time: Instant::now(),
                fetched: false,
            }),
        });

        let thread = Some(thread::spawn({
            let shared = shared.clone();
            move || {
                steamdeck_input_thread(shared);
            }
        }));

        SteamdeckInput { shared, thread }
    }

    pub fn fetch(&self) -> Option<GamepadState> {
        if self.shared.found.load(Ordering::SeqCst) {
            self.shared.state.lock().unwrap().fetch()
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
