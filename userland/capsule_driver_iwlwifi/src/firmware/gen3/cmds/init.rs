// NONOS Operating System (AGPL-3.0-or-later)

//! The payloads the bring-up sends after ALIVE, before the regulatory domain.

fn le32s(words: &[u32]) -> alloc::vec::Vec<u8> {
    words.iter().flat_map(|w| w.to_le_bytes()).collect()
}

/// `iwl_init_extended_cfg_cmd`: the driver will send NVM access commands
/// (`BIT(IWL_INIT_NVM)`).
pub fn init_extended_cfg() -> [u8; 4] {
    (1u32 << 1).to_le_bytes()
}

/// `iwl_nvm_access_complete_cmd` and `iwl_nvm_get_info`: one reserved word.
pub fn reserved_word() -> [u8; 4] {
    [0; 4]
}

/// `iwl_tx_ant_cfg_cmd`.
pub fn tx_ant_cfg(valid_tx_ant: u8) -> [u8; 4] {
    (valid_tx_ant as u32).to_le_bytes()
}

/// `iwl_bt_coex_cmd` as `iwl_mvm_send_bt_init_conf` fills it: mode
/// `BT_COEX_NW`, sync-to-SCO and high-band retention, multi-priority LUT when
/// the firmware supports it.
pub fn bt_config(mplut: bool) -> alloc::vec::Vec<u8> {
    const BT_COEX_NW: u32 = 0x1;
    const MPLUT: u32 = 1 << 0;
    const SYNC2SCO: u32 = 1 << 2;
    const HIGH_BAND_RET: u32 = 1 << 4;
    let modules = SYNC2SCO | HIGH_BAND_RET | if mplut { MPLUT } else { 0 };
    le32s(&[BT_COEX_NW, modules])
}

/// `iwl_soc_configuration_cmd` as `iwl_set_soc_latency` fills it.
pub fn soc_configuration(
    integrated: bool,
    ltr_delay: u8,
    low_latency_xtal: bool,
    xtal_latency: u32,
) -> alloc::vec::Vec<u8> {
    const DISCRETE: u32 = 1 << 0;
    const LOW_LATENCY: u32 = 1 << 1;
    let mut flags = if integrated { 0 } else { DISCRETE };
    if integrated {
        flags |= (ltr_delay as u32 & 0x3) << 2;
    }
    // Set when SCAN_REQ_UMAC is version 2 or later, true for every gen3 image.
    if low_latency_xtal {
        flags |= LOW_LATENCY;
    }
    le32s(&[flags, xtal_latency])
}

/// `iwl_device_power_cmd`: power save off (the CAM scheme). The driver polls
/// and does not wake the device for each access, so it keeps it awake.
pub fn device_power() -> [u8; 4] {
    [0; 4]
}
