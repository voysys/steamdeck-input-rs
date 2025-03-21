use std::{fmt::Debug, mem};

use bytemuck::{Pod, Zeroable};
use static_assertions::const_assert_eq;

pub const VALVE_IN_REPORT_MSG_VERSION: u16 = 0x01;

pub const VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_STATE: u8 = 1;
pub const VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_DEBUG: u8 = 2;
pub const VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_WIRELESS: u8 = 3;
pub const VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_STATUS: u8 = 4;
pub const VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_DEBUG2: u8 = 5;
pub const VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_SECONDARY_STATE: u8 = 6;
pub const VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_BLE_STATE: u8 = 7;
pub const VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_DECK_STATE: u8 = 9;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct ValveInReportHeader {
    pub report_version: u16,
    pub report_type: u8,
    pub report_length: u8,
}

const_assert_eq!(mem::size_of::<ValveInReportHeader>(), 4);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct ValveControllerStatePacket {
    pub packet_num: u32,
    pub button_trigger_data: u64,
    pub left_pad_x: i16,
    pub left_pad_y: i16,
    pub right_pad_x: i16,
    pub right_pad_y: i16,
    pub trigger_l: u16,
    pub trigger_r: u16,
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
    pub gyro_quat_w: i16,
    pub gyro_quat_x: i16,
    pub gyro_quat_y: i16,
    pub gyro_quat_z: i16,
}

const_assert_eq!(mem::size_of::<ValveControllerStatePacket>(), 44);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct ValveControllerBLEStatePacket {
    pub packet_num: u32,
    pub button_trigger_data: u64,
    pub left_pad_x: i16,
    pub left_pad_y: i16,
    pub right_pad_x: i16,
    pub right_pad_y: i16,
    pub gyro_data_type: u8,
    pub gyro: [i16; 4],
}

const_assert_eq!(mem::size_of::<ValveControllerBLEStatePacket>(), 29);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct ValveControllerDebugPacket {
    pub left_pad_x: i16,
    pub left_pad_y: i16,
    pub right_pad_x: i16,
    pub right_pad_y: i16,
    pub left_pad_mouse_dx: i16,
    pub left_pad_mouse_dy: i16,
    pub right_pad_mouse_dx: i16,
    pub right_pad_mouse_dy: i16,
    pub left_pad_mouse_filtered_dx: i16,
    pub left_pad_mouse_filtered_dy: i16,
    pub right_pad_mouse_filtered_dx: i16,
    pub right_pad_mouse_filtered_dy: i16,
    pub left_z: u8,
    pub right_z: u8,
    pub left_finger_present: u8,
    pub right_finger_present: u8,
    pub left_timestamp: u8,
    pub right_timestamp: u8,
    pub left_tap_state: u8,
    pub right_tap_state: u8,
    pub digital_io_states0: u32,
    pub digital_io_states1: u32,
}

const_assert_eq!(mem::size_of::<ValveControllerDebugPacket>(), 40);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct ValveControllerTrackpadImage {
    pub pad_num: u8,
    pub pad: [u8; 3],
    pub data: [i16; 20],
    pub noise: u16,
}

const_assert_eq!(mem::size_of::<ValveControllerTrackpadImage>(), 46);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct ValveControllerRawTrackpadImage {
    pub pad_num: u8,
    pub offset: u8,
    pub pad: [u8; 2],
    pub data: [i16; 28],
}

const_assert_eq!(mem::size_of::<ValveControllerRawTrackpadImage>(), 60);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SteamControllerWirelessEvent {
    pub event_type: u8,
}

const_assert_eq!(mem::size_of::<SteamControllerWirelessEvent>(), 1);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SteamControllerStatusEvent {
    pub packet_num: u32,
    pub event_code: u16,
    pub state_flags: u16,
    pub battery_voltage: u16,
    pub battery_level: u8,
}

const_assert_eq!(mem::size_of::<SteamControllerStatusEvent>(), 11);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SteamDeckStatePacket {
    pub packet_num: u32,
    pub buttons: u64,
    pub left_pad_x: i16,
    pub left_pad_y: i16,
    pub right_pad_x: i16,
    pub right_pad_y: i16,
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
    pub gyro_quat_w: i16,
    pub gyro_quat_x: i16,
    pub gyro_quat_y: i16,
    pub gyro_quat_z: i16,
    pub trigger_raw_l: u16,
    pub trigger_raw_r: u16,
    pub left_stick_x: i16,
    pub left_stick_y: i16,
    pub right_stick_x: i16,
    pub right_stick_y: i16,
    pub pressure_pad_left: u16,
    pub pressure_pad_right: u16,
}

const_assert_eq!(mem::size_of::<SteamDeckStatePacket>(), 56);

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union ValveInReportPayload {
    pub controller_state: ValveControllerStatePacket,
    pub controller_ble_state: ValveControllerBLEStatePacket,
    pub debug_state: ValveControllerDebugPacket,
    pub pad_image: ValveControllerTrackpadImage,
    pub raw_pad_image: ValveControllerRawTrackpadImage,
    pub wireless_event: SteamControllerWirelessEvent,
    pub status_event: SteamControllerStatusEvent,
    pub deck_state: SteamDeckStatePacket,
}

const_assert_eq!(mem::size_of::<ValveInReportPayload>(), 60);

unsafe impl Zeroable for ValveInReportPayload {}
unsafe impl Pod for ValveInReportPayload {}

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct ValveInReport {
    pub header: ValveInReportHeader,
    pub payload: ValveInReportPayload,
}

const_assert_eq!(mem::size_of::<ValveInReport>(), 64);

impl ValveInReport {
    pub fn to_deck_state(&self) -> Result<SteamDeckStatePacket, String> {
        if self.header.report_version != VALVE_IN_REPORT_MSG_VERSION
            || self.header.report_type != VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_DECK_STATE
            || self.header.report_length != 64
        {
            let version = self.header.report_version;
            return Err(format!(
                "Got unknown steamdeck message: version: {version}, id: {} size: {}",
                self.header.report_type, self.header.report_length
            ));
        }

        Ok(unsafe { self.payload.deck_state })
    }
}

pub const HID_FEATURE_REPORT_BYTES: usize = 64;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct FeatureReportHeader {
    pub report_type: u8,
    pub report_length: u8,
}

const_assert_eq!(mem::size_of::<FeatureReportHeader>(), 2);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct ControllerSetting {
    pub setting_num: u8,
    pub setting_value: u16,
}

const_assert_eq!(mem::size_of::<ControllerSetting>(), 3);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct ControllerAttribute {
    pub attribute_tag: u8,
    pub attribute_value: u32,
}

const_assert_eq!(mem::size_of::<ControllerAttribute>(), 5);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MsgSettings {
    pub settings: [ControllerSetting; 20],
}

const_assert_eq!(mem::size_of::<MsgSettings>(), 60);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MsgGetAttributes {
    pub attributes: [ControllerAttribute; 12],
}

const_assert_eq!(mem::size_of::<MsgGetAttributes>(), 60);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MsgGetStringAttribute {
    pub attribute_tag: u8,
    pub attribute_value: [u8; 20],
}

const_assert_eq!(mem::size_of::<MsgGetStringAttribute>(), 21);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MsgSetControllerMode {
    pub mode: u8,
}

const_assert_eq!(mem::size_of::<MsgSetControllerMode>(), 1);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MsgFireHapticPulse {
    pub which_pad: u8,
    pub pulse_duration: u16,
    pub pulse_interval: u16,
    pub pulse_count: u16,
    pub db_gain: i16,
    pub priority: u8,
}

const_assert_eq!(mem::size_of::<MsgFireHapticPulse>(), 10);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MsgHapticSetMode {
    pub mode: u8,
}

const_assert_eq!(mem::size_of::<MsgHapticSetMode>(), 1);

pub const HAPTIC_TYPE_OFF: u8 = 0;
pub const HAPTIC_TYPE_TICK: u8 = 1;
pub const HAPTIC_TYPE_CLICK: u8 = 2;
pub const HAPTIC_TYPE_TONE: u8 = 3;
pub const HAPTIC_TYPE_RUMBLE: u8 = 4;
pub const HAPTIC_TYPE_NOISE: u8 = 5;
pub const HAPTIC_TYPE_SCRIPT: u8 = 6;
pub const HAPTIC_TYPE_LOG_SWEEP: u8 = 7;

pub const HAPTIC_INTENSITY_SYSTEM: u8 = 0;
pub const HAPTIC_INTENSITY_SHORT: u8 = 1;
pub const HAPTIC_INTENSITY_MEDIUM: u8 = 2;
pub const HAPTIC_INTENSITY_LONG: u8 = 3;
pub const HAPTIC_INTENSITY_INSANE: u8 = 4;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MsgTriggerHaptic {
    pub side: u8,
    pub cmd: u8,
    pub ui_intensity: u8,
    pub db_gain: i8,
    pub freq: u16,
    pub dur_ms: i16,
    pub noise_intensity: u16,
    pub lfo_freq: u16,
    pub lfo_depth: u8,
    pub rand_tone_gain: u8,
    pub script_id: u8,
    pub lss_start_freq: u16,
    pub lss_end_freq: u16,
}

const_assert_eq!(mem::size_of::<MsgTriggerHaptic>(), 19);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MsgSimpleRumbleCmd {
    pub rumble_type: u8,
    pub intensity: u16,
    pub left_motor_speed: u16,
    pub right_motor_speed: u16,
    pub left_gain: i8,
    pub right_gain: i8,
}

const_assert_eq!(mem::size_of::<MsgSimpleRumbleCmd>(), 9);

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union FeatureReportMsgPayload {
    pub set_settings_values: MsgSettings,
    pub get_settings_values: MsgSettings,
    pub get_settings_maxs: MsgSettings,
    pub get_settings_defaults: MsgSettings,
    pub get_attributes: MsgGetAttributes,
    pub controller_mode: MsgSetControllerMode,
    pub fire_haptic_pulse: MsgFireHapticPulse,
    pub get_string_attribute: MsgGetStringAttribute,
    pub haptic_hode: MsgHapticSetMode,
    pub trigger_haptic: MsgTriggerHaptic,
    pub simple_rumble: MsgSimpleRumbleCmd,
}

const_assert_eq!(mem::size_of::<FeatureReportMsgPayload>(), 60);

unsafe impl Zeroable for FeatureReportMsgPayload {}
unsafe impl Pod for FeatureReportMsgPayload {}

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct FeatureReportMsg {
    pub header: FeatureReportHeader,
    pub payload: FeatureReportMsgPayload,
}

const_assert_eq!(mem::size_of::<FeatureReportMsg>(), 62);

pub const FEATURE_REPORT_MESSAGE_ID_SET_DIGITAL_MAPPINGS: u8 = 0x80;
pub const FEATURE_REPORT_MESSAGE_ID_CLEAR_DIGITAL_MAPPINGS: u8 = 0x81;
pub const FEATURE_REPORT_MESSAGE_ID_GET_DIGITAL_MAPPINGS: u8 = 0x82;
pub const FEATURE_REPORT_MESSAGE_ID_GET_ATTRIBUTES_VALUES: u8 = 0x83;
pub const FEATURE_REPORT_MESSAGE_ID_GET_ATTRIBUTE_LABEL: u8 = 0x84;
pub const FEATURE_REPORT_MESSAGE_ID_SET_DEFAULT_DIGITAL_MAPPINGS: u8 = 0x85;
pub const FEATURE_REPORT_MESSAGE_ID_FACTORY_RESET: u8 = 0x86;
pub const FEATURE_REPORT_MESSAGE_ID_SET_SETTINGS_VALUES: u8 = 0x87;
pub const FEATURE_REPORT_MESSAGE_ID_CLEAR_SETTINGS_VALUES: u8 = 0x88;
pub const FEATURE_REPORT_MESSAGE_ID_GET_SETTINGS_VALUES: u8 = 0x89;
pub const FEATURE_REPORT_MESSAGE_ID_GET_SETTING_LABEL: u8 = 0x8A;
pub const FEATURE_REPORT_MESSAGE_ID_GET_SETTINGS_MAXS: u8 = 0x8B;
pub const FEATURE_REPORT_MESSAGE_ID_GET_SETTINGS_DEFAULTS: u8 = 0x8C;
pub const FEATURE_REPORT_MESSAGE_ID_SET_CONTROLLER_MODE: u8 = 0x8D;
pub const FEATURE_REPORT_MESSAGE_ID_LOAD_DEFAULT_SETTINGS: u8 = 0x8E;
pub const FEATURE_REPORT_MESSAGE_ID_TRIGGER_HAPTIC_PULSE: u8 = 0x8F;
pub const FEATURE_REPORT_MESSAGE_ID_TURN_OFF_CONTROLLER: u8 = 0x9F;
pub const FEATURE_REPORT_MESSAGE_ID_GET_DEVICE_INFO: u8 = 0xA1;
pub const FEATURE_REPORT_MESSAGE_ID_CALIBRATE_TRACKPADS: u8 = 0xA7;
pub const FEATURE_REPORT_MESSAGE_ID_RESERVED_0: u8 = 0xA8;
pub const FEATURE_REPORT_MESSAGE_ID_SET_SERIAL_NUMBER: u8 = 0xA9;
pub const FEATURE_REPORT_MESSAGE_ID_GET_TRACKPAD_CALIBRATION: u8 = 0xAA;
pub const FEATURE_REPORT_MESSAGE_ID_GET_TRACKPAD_FACTORY_CALIBRATION: u8 = 0xAB;
pub const FEATURE_REPORT_MESSAGE_ID_GET_TRACKPAD_RAW_DATA: u8 = 0xAC;
pub const FEATURE_REPORT_MESSAGE_ID_ENABLE_PAIRING: u8 = 0xAD;
pub const FEATURE_REPORT_MESSAGE_ID_GET_STRING_ATTRIBUTE: u8 = 0xAE;
pub const FEATURE_REPORT_MESSAGE_ID_RADIO_ERASE_RECORDS: u8 = 0xAF;
pub const FEATURE_REPORT_MESSAGE_ID_RADIO_WRITE_RECORD: u8 = 0xB0;
pub const FEATURE_REPORT_MESSAGE_ID_SET_DONGLE_SETTING: u8 = 0xB1;
pub const FEATURE_REPORT_MESSAGE_ID_DONGLE_DISCONNECT_DEVICE: u8 = 0xB2;
pub const FEATURE_REPORT_MESSAGE_ID_DONGLE_COMMIT_DEVICE: u8 = 0xB3;
pub const FEATURE_REPORT_MESSAGE_ID_DONGLE_GET_WIRELESS_STATE: u8 = 0xB4;
pub const FEATURE_REPORT_MESSAGE_ID_CALIBRATE_GYRO: u8 = 0xB5;
pub const FEATURE_REPORT_MESSAGE_ID_PLAY_AUDIO: u8 = 0xB6;
pub const FEATURE_REPORT_MESSAGE_ID_AUDIO_UPDATE_START: u8 = 0xB7;
pub const FEATURE_REPORT_MESSAGE_ID_AUDIO_UPDATE_DATA: u8 = 0xB8;
pub const FEATURE_REPORT_MESSAGE_ID_AUDIO_UPDATE_COMPLETE: u8 = 0xB9;
pub const FEATURE_REPORT_MESSAGE_ID_GET_CHIPID: u8 = 0xBA;
pub const FEATURE_REPORT_MESSAGE_ID_CALIBRATE_JOYSTICK: u8 = 0xBF;
pub const FEATURE_REPORT_MESSAGE_ID_CALIBRATE_ANALOG_TRIGGERS: u8 = 0xC0;
pub const FEATURE_REPORT_MESSAGE_ID_SET_AUDIO_MAPPING: u8 = 0xC1;
pub const FEATURE_REPORT_MESSAGE_ID_CHECK_GYRO_FW_LOAD: u8 = 0xC2;
pub const FEATURE_REPORT_MESSAGE_ID_CALIBRATE_ANALOG: u8 = 0xC3;
pub const FEATURE_REPORT_MESSAGE_ID_DONGLE_GET_CONNECTED_SLOTS: u8 = 0xC4;
pub const FEATURE_REPORT_MESSAGE_ID_RESET_IMU: u8 = 0xCE;
pub const FEATURE_REPORT_MESSAGE_ID_TRIGGER_HAPTIC_CMD: u8 = 0xEA;
pub const FEATURE_REPORT_MESSAGE_ID_TRIGGER_RUMBLE_CMD: u8 = 0xEB;

pub const BUTTON_RIGHT_TRIGGER: u64 = 1;
pub const BUTTON_LEFT_TRIGGER: u64 = 2;
pub const BUTTON_RIGHT_SHOLDER: u64 = 4;
pub const BUTTON_LEFT_SHOLDER: u64 = 8;
pub const BUTTON_R4: u64 = 0x40000000000;
pub const BUTTON_R5: u64 = 0x10000;
pub const BUTTON_L4: u64 = 0x20000000000;
pub const BUTTON_L5: u64 = 0x8000;
pub const BUTTON_DPAD_UP: u64 = 256;
pub const BUTTON_DPAD_DOWN: u64 = 2048;
pub const BUTTON_DPAD_LEFT: u64 = 1024;
pub const BUTTON_DPAD_RIGHT: u64 = 512;
pub const BUTTON_A: u64 = 128;
pub const BUTTON_B: u64 = 32;
pub const BUTTON_X: u64 = 64;
pub const BUTTON_Y: u64 = 16;
pub const BUTTON_VIEW: u64 = 0x1000;
pub const BUTTON_MENU: u64 = 0x4000;
pub const BUTON_STEAM: u64 = 0x2000;
pub const BUTON_QUICK_ACCESS: u64 = 0x4000000000000;
pub const BUTTON_RIGHT_STICK: u64 = 0x4000000;
pub const BUTTON_LEFT_STICK: u64 = 0x400000;
pub const RIGHT_STICK_USED: u64 = 0x800000000000;
pub const LEFT_STICK_USED: u64 = 0x400000000000;
