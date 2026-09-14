use super::*;
use bytemuck::Zeroable;
use std::{cell::Cell, collections::VecDeque};

fn state() -> GamepadUpdateState {
    GamepadUpdateState {
        gamepad: GamepadState::default(),
        last_update_time: Instant::now(),
        fetched: true,
    }
}

#[test]
fn original_button_indices_and_axes_are_preserved() {
    let buttons = [
        BUTTON_A,
        BUTTON_B,
        BUTTON_X,
        BUTTON_Y,
        BUTTON_LEFT_BUMPER,
        BUTTON_RIGHT_BUMPER,
        BUTTON_VIEW,
        BUTTON_MENU,
        BUTTON_QUICK_ACCESS,
        BUTTON_LEFT_STICK,
        BUTTON_RIGHT_STICK,
        BUTTON_DPAD_UP,
        BUTTON_DPAD_RIGHT,
        BUTTON_DPAD_DOWN,
        BUTTON_DPAD_LEFT,
        BUTTON_R4,
        BUTTON_R5,
        BUTTON_L4,
        BUTTON_L5,
        BUTTON_STEAM,
        BUTTON_LEFT_PAD,
        BUTTON_RIGHT_PAD,
    ];
    for (index, button) in buttons.into_iter().enumerate() {
        let mut state = state();
        let mut packet = SteamDeckStatePacket::zeroed();
        packet.buttons = button;
        packet.left_stick_x = i16::MIN;
        packet.left_stick_y = i16::MAX;
        packet.right_stick_x = i16::MAX;
        packet.right_stick_y = i16::MIN;
        packet.trigger_raw_r = i16::MAX as u16;
        state.update(&packet);
        let result = state.fetch().unwrap();
        assert_eq!(result.buttons.iter().sum::<u8>(), 1);
        assert_eq!(result.buttons[index], 1);
        assert_eq!(result.axes, [-1.0, -1.0, 1.0, 1.0, -1.0, 1.0]);
    }
}

#[test]
fn presence_checks_preserve_short_presses_until_fetch_and_loss_clears_them() {
    let reader = SteamdeckInput {
        shared: Arc::new(SteamdeckShared {
            run: AtomicBool::new(true),
            state: Mutex::new(None),
        }),
        thread: None,
    };
    assert!(reader.peek().is_none());
    let mut value = state();
    let mut packet = SteamDeckStatePacket::zeroed();
    packet.buttons = BUTTON_QUICK_ACCESS;
    value.update(&packet);
    packet.buttons = 0;
    value.update(&packet);
    *reader.shared.state.lock().unwrap() = Some(value);
    assert_eq!(reader.peek().unwrap().buttons[8], 1);
    assert_eq!(reader.peek().unwrap().buttons[8], 1);
    assert_eq!(reader.fetch().unwrap().buttons[8], 1);
    reader
        .shared
        .state
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .update(&packet);
    assert_eq!(reader.fetch().unwrap().buttons[8], 0);
    reader
        .shared
        .state
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .last_update_time = Instant::now() - Duration::from_secs(1);
    assert!(reader.fetch().is_none());
    assert!(reader.shared.state.lock().unwrap().is_none());
}

#[test]
fn malformed_reports_are_fallible_and_unrelated_reports_are_ignored() {
    for length in 0..256 {
        assert!(ValveInReport::parse_deck_state(&vec![0; length]).is_err());
    }
    let mut packet = [0; 64];
    packet[..4].copy_from_slice(&[1, 0, 4, 11]);
    assert!(ValveInReport::parse_deck_state(&packet).unwrap().is_none());
    packet[2] = 9;
    assert!(ValveInReport::parse_deck_state(&packet).is_err());
    packet[3] = 64;
    assert!(ValveInReport::parse_deck_state(&packet).unwrap().is_some());
}

struct FakeController {
    reads: Mutex<VecDeque<Vec<u8>>>,
    configurations: Cell<usize>,
}

impl Controller for FakeController {
    fn read(&self, bytes: &mut [u8]) -> HidResult<usize> {
        match self.reads.lock().unwrap().pop_front() {
            Some(report) => {
                bytes[..report.len()].copy_from_slice(&report);
                Ok(report.len())
            }
            None => Err(HidError::HidApiError {
                message: "device disconnected".into(),
            }),
        }
    }
    fn configure(&self) -> HidResult<()> {
        self.configurations.set(self.configurations.get() + 1);
        Ok(())
    }
}

#[test]
fn timeouts_and_other_reports_keep_one_session_and_disconnect_clears_state() {
    let shared = SteamdeckShared {
        run: AtomicBool::new(true),
        state: Mutex::new(None),
    };
    let mut valid = vec![0; 64];
    valid[..4].copy_from_slice(&[1, 0, 9, 64]);
    valid[8] = BUTTON_A as u8;
    let mut other = vec![0; 64];
    other[..4].copy_from_slice(&[1, 0, 4, 11]);
    let mut reads = VecDeque::from([valid.clone(), valid.clone()]);
    reads.extend(vec![Vec::new(); 1000]);
    reads.extend([vec![1, 2], other, valid]);
    let fake = FakeController {
        reads: Mutex::new(reads),
        configurations: Cell::new(0),
    };
    assert!(read_device(&shared, &fake).is_err());
    assert!(fake.reads.lock().unwrap().is_empty());
    assert_eq!(fake.configurations.get(), 1);
    assert!(shared.state.lock().unwrap().is_none());
}

#[test]
fn shutdown_joins_a_worker_waiting_for_retry() {
    let shared = Arc::new(SteamdeckShared {
        run: AtomicBool::new(true),
        state: Mutex::new(None),
    });
    let worker_shared = Arc::clone(&shared);
    let thread = thread::spawn(move || {
        while worker_shared.run.load(Ordering::Relaxed) {
            thread::park_timeout(Duration::from_secs(60));
        }
    });
    let reader = SteamdeckInput {
        shared: Arc::clone(&shared),
        thread: Some(thread),
    };
    drop(reader);
    assert!(!shared.run.load(Ordering::Relaxed));
}
