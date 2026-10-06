// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The host commands the bring-up sends after ALIVE, with the payloads Linux
//! v6.12 builds for this firmware (mvm/fw.c `iwl_run_unified_mvm_ucode` and
//! `iwl_mvm_up`, mvm/coex.c, fw/init.c, mvm/power.c, mvm/nvm.c, mvm/scan.c,
//! mvm/mld-mac.c, mvm/link.c). Each payload is a little-endian byte image of
//! its `struct iwl_*`; the sizes and field offsets are pinned in the proofs to
//! numbers compiled from the Linux headers. Legacy commands go out in
//! `LONG_GROUP`, as `iwl_trans_send_cmd` maps them with the wide header.

/// Command groups (fw/api/commands.h).
pub const LEGACY_GROUP: u8 = 0x0;
pub const LONG_GROUP: u8 = 0x1;
pub const SYSTEM_GROUP: u8 = 0x2;
pub const MAC_CONF_GROUP: u8 = 0x3;
pub const REGULATORY_AND_NVM_GROUP: u8 = 0xC;

pub const INIT_COMPLETE_NOTIF: u8 = 0x04;
pub const SOC_CONFIGURATION_CMD: u8 = 0x01;
pub const INIT_EXTENDED_CFG_CMD: u8 = 0x03;
pub const NVM_ACCESS_COMPLETE: u8 = 0x00;
pub const NVM_GET_INFO: u8 = 0x02;
pub const PNVM_INIT_COMPLETE_NTFY: u8 = 0xFE;
pub const TX_ANT_CONFIGURATION_CMD: u8 = 0x98;
pub const BT_CONFIG: u8 = 0x9B;
pub const POWER_TABLE_CMD: u8 = 0x77;
pub const MCC_UPDATE_CMD: u8 = 0xC8;
pub const SCAN_CFG_CMD: u8 = 0x0C;
pub const SCAN_REQ_UMAC: u8 = 0x0D;
pub const SCAN_COMPLETE_UMAC: u8 = 0x0F;
pub const MAC_CONFIG_CMD: u8 = 0x08;
pub const LINK_CONFIG_CMD: u8 = 0x09;
pub const REPLY_RX_MPDU_CMD: u8 = 0xC1;

/// Capability and API bits this path reads (fw/file.h).
pub const CAPA_LAR_SUPPORT: u32 = 1;
pub const CAPA_UMAC_SCAN: u32 = 2;
pub const CAPA_SOC_LATENCY_SUPPORT: u32 = 37;
pub const CAPA_BT_MPLUT_SUPPORT: u32 = 67;
pub const CAPA_MCC_UPDATE_11AX_SUPPORT: u32 = 89;
pub const CAPA_MLD_API_SUPPORT: u32 = 110;
pub const API_REGULATORY_NVM_INFO: u32 = 48;

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

/// `iwl_mcc_update_cmd` asking for the world domain "ZZ" from the firmware's
/// own data (`MCC_SOURCE_OLD_FW`), as `iwl_mvm_init_mcc` does on a platform
/// without a BIOS country code.
pub fn mcc_update_world() -> [u8; 28] {
    let mut c = [0u8; 28];
    c[0..2].copy_from_slice(&(((b'Z' as u16) << 8) | b'Z' as u16).to_le_bytes());
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

/// `FW_CTXT_ACTION_ADD`, `FW_MAC_TYPE_BSS_STA`, `FW_CTXT_INVALID`.
const ACTION_ADD: u32 = 1;
const MAC_TYPE_BSS_STA: u32 = 5;
const CTXT_INVALID: u32 = 0xFFFF_FFFF;

/// `iwl_mac_config_cmd` adding station MAC 0 at `addr`, unassociated: accept
/// group frames and beacons; HE off until association; the NIC does not
/// advertise ACK-enabled aggregation (its HE capabilities do not set it).
pub fn mac_config_add(addr: [u8; 6]) -> [u8; 52] {
    const ACCEPT_GRP: u32 = 1 << 2;
    const ACCEPT_BEACON: u32 = 1 << 3;
    let mut c = [0u8; 52];
    c[4..8].copy_from_slice(&ACTION_ADD.to_le_bytes());
    c[8..12].copy_from_slice(&MAC_TYPE_BSS_STA.to_le_bytes());
    c[12..18].copy_from_slice(&addr);
    c[20..24].copy_from_slice(&(ACCEPT_GRP | ACCEPT_BEACON).to_le_bytes());
    c[32..36].copy_from_slice(&1u32.to_le_bytes());
    c
}

/// `iwl_link_config_cmd` adding link 0 of MAC 0 with no PHY context yet.
pub fn link_config_add(addr: [u8; 6]) -> alloc::vec::Vec<u8> {
    let mut c = alloc::vec![0u8; 208];
    c[0..4].copy_from_slice(&ACTION_ADD.to_le_bytes());
    c[12..16].copy_from_slice(&CTXT_INVALID.to_le_bytes());
    c[16..22].copy_from_slice(&addr);
    c
}
