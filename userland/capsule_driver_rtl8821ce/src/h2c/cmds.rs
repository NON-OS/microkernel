// NONOS Operating System (AGPL-3.0-or-later)

//! The firmware commands power save sends, laid out as rtw88 lays them
//! (`fw.h`), each the two words the mailbox takes: the first four bytes,
//! then the next.

const SET_PWR_MODE: u32 = 0x20;

/// Power states the firmware is told to hold (rtw88 `enum rtw_pwr_state`).
const RF_OFF: u32 = 0x0;
const ALL_ON: u32 = 0xC;

/// Leisure power save as rtw88 enters it (mode LPS, RF off between beacons,
/// awake for every beacon, smart PS 2), or all on as it leaves (mode active).
/// Port 0.
pub fn pwr_mode(lps: bool) -> (u32, u32) {
    let (mode, rlbm, smart_ps, state) = if lps { (1, 1, 2, RF_OFF) } else { (0, 0, 0, ALL_ON) };
    let awake_interval = 1;
    let w0 = SET_PWR_MODE | mode << 8 | rlbm << 16 | smart_ps << 20 | awake_interval << 24;
    (w0, state << 8)
}

const RSVD_PAGE: u32 = 0x00;
const MEDIA_STATUS_RPT: u32 = 0x01;

/// The station on `mac_id` connected or not: the firmware's power save acts
/// only for a connected one.
pub fn media_status(connected: bool, mac_id: u8) -> (u32, u32) {
    (MEDIA_STATUS_RPT | u32::from(connected) << 8 | u32::from(mac_id) << 16, 0)
}

/// Where the reserved pages hold the frames the firmware sends on its own,
/// counted from the driver's first page; no probe response.
pub fn rsvd_page(ps_poll: u8, null: u8, qos_null: u8) -> (u32, u32) {
    (RSVD_PAGE | u32::from(ps_poll) << 16 | u32::from(null) << 24, u32::from(qos_null))
}
