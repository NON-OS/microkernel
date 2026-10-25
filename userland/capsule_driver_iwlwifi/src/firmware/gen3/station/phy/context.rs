// NONOS Operating System (AGPL-3.0-or-later)

//! The PHY context command and its band.

/// `FW_CTXT_ACTION_ADD`, `_MODIFY`, `_REMOVE` (fw/api/mac.h).
pub const ACTION_ADD: u32 = 1;
pub const ACTION_MODIFY: u32 = 2;
pub const ACTION_REMOVE: u32 = 3;

/// The size of each command.
pub const PHY_CONTEXT_LEN: usize = 32;
pub const RLC_CONFIG_LEN: usize = 32;

/// The one PHY context this driver uses.
pub const PHY_ID: u32 = 0;
const PHY_BAND_5: u8 = 0;
const PHY_BAND_24: u8 = 1;
const PHY_CHANNEL_MODE20: u8 = 0;
pub(super) const RX_CHAIN_VALID_POS: u32 = 1;

/// The band a 2.4 or 5 GHz channel number is in.
pub fn band(channel: u8) -> u8 {
    if channel <= 14 {
        PHY_BAND_24
    } else {
        PHY_BAND_5
    }
}

/// `iwl_phy_context_cmd` for PHY 0 on `channel` at 20 MHz.
pub fn phy_context(action: u32, channel: u8) -> [u8; PHY_CONTEXT_LEN] {
    let mut c = [0u8; PHY_CONTEXT_LEN];
    c[0..4].copy_from_slice(&PHY_ID.to_le_bytes());
    c[4..8].copy_from_slice(&action.to_le_bytes());
    c[8..12].copy_from_slice(&u32::from(channel).to_le_bytes());
    c[12] = band(channel);
    c[13] = PHY_CHANNEL_MODE20;
    c
}
