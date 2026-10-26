// NONOS Operating System (AGPL-3.0-or-later)

//! The registers power save writes besides the firmware's mailbox.

/// The Wi-Fi to Bluetooth scoreboard (rtw88 REG_WIFI_BT_INFO), as the
/// coexistence setup leaves it: Wi-Fi active and on, with the interrupt to
/// Bluetooth enabled (`coex::wl_only`). Asleep, active is cleared, as rtw88's
/// LPS notice does (`rtw_coex_lps_notify`); busy was never set here.
pub const REG_WIFI_BT_INFO: usize = 0x00AA;
pub const SCBD_AWAKE: u16 = 0x8000 | 0x0003;
pub const SCBD_ASLEEP: u16 = 0x8000 | 0x0002;

/// The transmit configuration: while the firmware sleeps it has the hardware
/// mark data frames with the power management bit, and it clears this again
/// once the access point has heard the station wake (rtw88 BIT_PWRMGT_HWDATA_EN).
pub const REG_TCR: usize = 0x0604;
pub const PWRMGT_HWDATA_EN: u32 = 1 << 7;
/// How long the firmware is given to tell the access point (rtw88: five tries
/// twenty milliseconds apart).
pub const LEAVE_MS: u64 = 100;
