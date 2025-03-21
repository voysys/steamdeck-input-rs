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

#[derive(Copy, Clone, Debug)]
pub enum ValveReport {
    ValveControllerStatePacket(ValveControllerStatePacket),
    ValveControllerBLEStatePacket(ValveControllerBLEStatePacket),
    ValveControllerDebugPacket(ValveControllerDebugPacket),
    ValveControllerTrackpadImage(ValveControllerTrackpadImage),
    ValveControllerRawTrackpadImage(ValveControllerRawTrackpadImage),
    SteamControllerWirelessEvent(SteamControllerWirelessEvent),
    SteamControllerStatusEvent(SteamControllerStatusEvent),
    SteamDeckStatePacket(SteamDeckStatePacket),
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
    pub fn as_enum(&self) -> Result<ValveReport, String> {
        if self.header.report_version != VALVE_IN_REPORT_MSG_VERSION
            || self.header.report_length != 64
        {
            let version = self.header.report_version;
            return Err(format!(
                "Got unknown steamdeck message: version: {version}, id: {} size: {}",
                self.header.report_type, self.header.report_length
            ));
        }

        unsafe {
            Ok(match self.header.report_type {
                VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_STATE => {
                    ValveReport::ValveControllerStatePacket(self.payload.controller_state)
                }
                VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_DEBUG => {
                    ValveReport::ValveControllerDebugPacket(self.payload.debug_state)
                }
                VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_WIRELESS => {
                    ValveReport::SteamControllerWirelessEvent(self.payload.wireless_event)
                }
                VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_STATUS => {
                    ValveReport::SteamControllerStatusEvent(self.payload.status_event)
                }
                VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_BLE_STATE => {
                    ValveReport::ValveControllerBLEStatePacket(self.payload.controller_ble_state)
                }
                VALVE_IN_REPORT_MESSAGE_ID_CONTROLLER_DECK_STATE => {
                    ValveReport::SteamDeckStatePacket(self.payload.deck_state)
                }
                _ => {
                    let version = self.header.report_version;
                    return Err(format!(
                        "Got unknown steamdeck message: version: {version}, id: {} size: {}",
                        self.header.report_type, self.header.report_length
                    ));
                }
            })
        }
    }
}
