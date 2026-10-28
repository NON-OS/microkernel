// NONOS Operating System (AGPL-3.0-or-later)

//! The world regulatory domain asked for and its reply read, and the scan configuration.

/// `iwl_mcc_update_cmd` for "ZZ" as `iwl_mvm_init_mcc` sends it on a
/// platform without a BIOS country code (`iwl_mvm_get_current_regdomain`):
/// to a firmware that takes Wi-Fi MCC updates (`wifi_mcc`, the
/// `iwl_mvm_is_wifi_mcc_supported` bits) from `MCC_SOURCE_GET_CURRENT`,
/// which asks its current domain and tells it the driver takes location
/// updates; to an older one from `MCC_SOURCE_OLD_FW`.
pub fn mcc_update_zz(wifi_mcc: bool) -> [u8; 28] {
    const MCC_SOURCE_GET_CURRENT: u8 = 0x10;
    let mut c = [0u8; 28];
    c[0..2].copy_from_slice(&(((b'Z' as u16) << 8) | b'Z' as u16).to_le_bytes());
    c[2] = if wifi_mcc { MCC_SOURCE_GET_CURRENT } else { 0 };
    c
}

/// The status and country of an MCC update reply, or `None` if its length
/// does not match its channel count. The layout follows the reply version:
/// 8 and up, the 11ax layout (4), or the older one (3).
pub fn mcc_update_reply(payload: &[u8], notif_ver: u8, ax_layout: bool) -> Option<(u32, u16)> {
    let n_at = if notif_ver >= 8 {
        20
    } else if ax_layout {
        16
    } else {
        12
    };
    let n = u32::from_le_bytes(payload.get(n_at..n_at + 4)?.try_into().ok()?) as usize;
    let want = n.checked_mul(4)?.checked_add(n_at + 4)?;
    if payload.len() != want {
        return None;
    }
    let status = u32::from_le_bytes(payload[0..4].try_into().ok()?);
    let mcc = u16::from_le_bytes([payload[4], payload[5]]);
    Some((status, mcc))
}

/// `iwl_scan_config` (version 5, the reduced config): no broadcast station,
/// the valid chains.
pub fn scan_config(tx_chains: u8, rx_chains: u8) -> [u8; 12] {
    let mut c = [0u8; 12];
    c[4..8].copy_from_slice(&(tx_chains as u32).to_le_bytes());
    c[8..12].copy_from_slice(&(rx_chains as u32).to_le_bytes());
    c
}
