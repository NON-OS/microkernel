// NONOS Operating System (AGPL-3.0-or-later)

//! The station's power save: `struct iwl_mac_power_cmd` (fw/api/power.h),
//! 40 bytes: `id_and_color` 0, `flags` 4, `keep_alive_seconds` 6,
//! `rx_data_timeout` 8, `tx_data_timeout` 12, the uAPSD timeouts 16 and 20,
//! then the LPRX threshold, DTIM skip, snooze, uAPSD and heavy-traffic
//! fields, all zero here. Filled as Linux v6.12 mvm/power.c
//! `iwl_mvm_power_build_cmd` fills it for a lone station under the default
//! (balanced) scheme: the keep-alive period at least three DTIMs and never
//! under 25 s, `POWER_SAVE_ENA` always, and once power management is on,
//! `POWER_MANAGEMENT_ENA` with the 100 ms default receive and transmit
//! timeouts. No uAPSD (Linux leaves it off for a BSS by default), no low
//! power receive (a station's beacon rate is unset) and no DTIM skip (only
//! the low-power scheme and suspend skip DTIMs).

use super::link::MAC_ID;

pub const MAC_POWER_LEN: usize = 40;
/// `POWER_FLAGS_POWER_SAVE_ENA_MSK` and `_POWER_MANAGEMENT_ENA_MSK`.
const POWER_SAVE_ENA: u16 = 1 << 0;
const POWER_MANAGEMENT_ENA: u16 = 1 << 1;
/// `POWER_KEEP_ALIVE_PERIOD_SEC`.
const KEEP_ALIVE_MIN_S: u64 = 25;
/// `IWL_MVM_DEFAULT_PS_RX_DATA_TIMEOUT` and `_TX_`, in microseconds.
const PS_DATA_TIMEOUT_US: u32 = 100_000;

/// The keep-alive period for a BSS of `dtim_period` and `beacon_int` (TU):
/// three DTIM intervals, rounded up to a second, and at least 25 s.
pub fn keep_alive_s(dtim_period: u8, beacon_int: u16) -> u16 {
    let us = 3 * u64::from(dtim_period) * u64::from(beacon_int) * 1024;
    let s = us.div_ceil(1_000_000).max(KEEP_ALIVE_MIN_S);
    u16::try_from(s).unwrap_or(u16::MAX)
}

/// `iwl_mac_power_cmd` for MAC 0 (color 0): power save allowed, and power
/// management on when `managed`.
pub fn mac_power(keep_alive: u16, managed: bool) -> [u8; MAC_POWER_LEN] {
    let mut c = [0u8; MAC_POWER_LEN];
    c[0..4].copy_from_slice(&MAC_ID.to_le_bytes());
    let flags = if managed { POWER_SAVE_ENA | POWER_MANAGEMENT_ENA } else { POWER_SAVE_ENA };
    c[4..6].copy_from_slice(&flags.to_le_bytes());
    c[6..8].copy_from_slice(&keep_alive.to_le_bytes());
    if managed {
        c[8..12].copy_from_slice(&PS_DATA_TIMEOUT_US.to_le_bytes());
        c[12..16].copy_from_slice(&PS_DATA_TIMEOUT_US.to_le_bytes());
    }
    c
}
