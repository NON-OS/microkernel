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
