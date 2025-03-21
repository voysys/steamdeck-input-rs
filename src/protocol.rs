use std::mem;

use bytemuck::{Pod, Zeroable};
use static_assertions::const_assert_eq;

pub const K_VALVE_IN_REPORT_MSG_VERSION: u8 = 0x01;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ValveInReportMessageIDs {
    ControllerState = 1,
    ControllerDebug = 2,
    ControllerWireless = 3,
    ControllerStatus = 4,
    ControllerDebug2 = 5,
    ControllerSecondaryState = 6,
    ControllerBleState = 7,
    ControllerDeckState = 9,
    ControllerMsgCount,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct ValveInReportHeader {
    pub report_version: u16,
    pub report_type: u8,
    pub report_length: u8,
}

const_assert_eq!(mem::size_of::<ValveInReportHeader>(), 4);

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct ButtonTriggers {
    pub pad0: [u8; 3],
    pub n_left: u8,
    pub n_right: u8,
    pub pad1: [u8; 3],
}

const_assert_eq!(mem::size_of::<ButtonTriggers>(), 8);

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union ButtonTriggerData {
    pub buttons: u64,
    pub triggers: ButtonTriggers,
}

const_assert_eq!(mem::size_of::<ButtonTriggerData>(), 8);

unsafe impl Zeroable for ButtonTriggerData {}
unsafe impl Pod for ButtonTriggerData {}

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct ValveControllerStatePacket {
    pub packet_num: u32,
    pub button_trigger_data: ButtonTriggerData,
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
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct ValveControllerBLEStatePacket {
    pub packet_num: u32,
    pub button_trigger_data: ButtonTriggerData,
    pub left_pad_x: i16,
    pub left_pad_y: i16,
    pub right_pad_x: i16,
    pub right_pad_y: i16,
    pub gyro_data_type: u8,
    pub gyro: [i16; 4],
}

const_assert_eq!(mem::size_of::<ValveControllerBLEStatePacket>(), 29);

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
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
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct ValveControllerTrackpadImage {
    pub pad_num: u8,
    pub pad: [u8; 3],
    pub data: [i16; 20],
    pub noise: u16,
}

const_assert_eq!(mem::size_of::<ValveControllerTrackpadImage>(), 46);

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct ValveControllerRawTrackpadImage {
    pub pad_num: u8,
    pub offset: u8,
    pub pad: [u8; 2],
    pub data: [i16; 28],
}

const_assert_eq!(mem::size_of::<ValveControllerRawTrackpadImage>(), 60);

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct SteamControllerWirelessEvent {
    pub event_type: u8,
}

const_assert_eq!(mem::size_of::<SteamControllerWirelessEvent>(), 1);

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct SteamControllerStatusEvent {
    pub packet_num: u32,
    pub event_code: u16,
    pub state_flags: u16,
    pub battery_voltage: u16,
    pub battery_level: u8,
}

const_assert_eq!(mem::size_of::<SteamControllerStatusEvent>(), 11);

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct DeckStateButtonHalves {
    pub buttons_l: u32,
    pub buttons_h: u32,
}

const_assert_eq!(mem::size_of::<DeckStateButtonHalves>(), 8);

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union DeckStateButtons {
    pub buttons: u64,
    pub halves: DeckStateButtonHalves,
}

const_assert_eq!(mem::size_of::<DeckStateButtons>(), 8);

unsafe impl Zeroable for DeckStateButtons {}
unsafe impl Pod for DeckStateButtons {}

#[repr(C, packed)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct SteamDeckStatePacket {
    pub packet_num: u32,
    pub buttons: DeckStateButtons,
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
