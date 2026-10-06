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

//! The groups and opcodes of the join's commands (fw/api/commands.h,
//! mac-cfg.h, datapath.h) and the version of each one's layout this driver
//! encodes. A firmware file says, per command, which version it runs
//! (`IWL_UCODE_TLV_FW_CMD_VERSIONS`); a command it does not list runs the
//! version Linux falls back to. [`check`] holds the loaded firmware to this
//! table before anything is sent, so a firmware that moved a layout is
//! refused instead of handed bytes it reads differently. The versions are
//! the ones both bundled SO images report.

use super::super::cmds::{LINK_CONFIG_CMD, LONG_GROUP, MAC_CONFIG_CMD, MAC_CONF_GROUP};
use super::super::ucode::Ucode;

/// `DATA_PATH_GROUP`.
pub const DATA_PATH_GROUP: u8 = 0x5;

/// `PHY_CONTEXT_CMD` (legacy, sent wide in `LONG_GROUP`).
pub const PHY_CONTEXT_CMD: u8 = 0x08;
/// `SCAN_ABORT_UMAC`.
pub const SCAN_ABORT_UMAC: u8 = 0x0E;
/// `TX_CMD`: a frame on a transmit queue, and the firmware's response to it.
pub const TX_CMD: u8 = 0x1C;
/// `TXPATH_FLUSH`.
pub const TXPATH_FLUSH: u8 = 0x1E;
/// `SESSION_PROTECTION_CMD` and its notification, in `MAC_CONF_GROUP`.
pub const SESSION_PROTECTION_CMD: u8 = 0x05;
pub const SESSION_PROTECTION_NOTIF: u8 = 0xFB;
/// `STA_CONFIG_CMD` and `STA_REMOVE_CMD`, in `MAC_CONF_GROUP`.
pub const STA_CONFIG_CMD: u8 = 0x0A;
pub const STA_REMOVE_CMD: u8 = 0x0C;
/// `RLC_CONFIG_CMD`, `SCD_QUEUE_CONFIG_CMD` and `SEC_KEY_CMD`, in
/// `DATA_PATH_GROUP`.
pub const RLC_CONFIG_CMD: u8 = 0x08;
pub const SCD_QUEUE_CONFIG_CMD: u8 = 0x17;
pub const SEC_KEY_CMD: u8 = 0x18;

/// One command and the versions whose layout this driver encodes. An empty
/// `listed` means the firmware must not list the command at all (or list it
/// as version 1): Linux then sends the version 1 layout.
pub struct Api {
    pub group: u8,
    pub cmd: u8,
    pub listed: &'static [u8],
}

/// Every command the join sends.
pub const APIS: [Api; 12] = [
    // iwl_phy_context_cmd, PHY_CONTEXT_CMD_API_VE_4.
    Api { group: LONG_GROUP, cmd: PHY_CONTEXT_CMD, listed: &[4] },
    // iwl_umac_scan_abort, version 1.
    Api { group: LONG_GROUP, cmd: SCAN_ABORT_UMAC, listed: &[1] },
    // iwl_tx_cmd_gen3, TX_CMD_API_S_VER_10, with the version 2 rate format.
    Api { group: LONG_GROUP, cmd: TX_CMD, listed: &[10] },
    // iwl_tx_path_flush_cmd, TX_PATH_FLUSH_CMD_API_S_VER_2.
    Api { group: LONG_GROUP, cmd: TXPATH_FLUSH, listed: &[2] },
    // iwl_mvm_session_prot_cmd, SESSION_PROTECTION_CMD_API_S_VER_1 and _2:
    // the same bytes; version 1 names MAC 0, version 2 link 0, both id 0.
    Api { group: MAC_CONF_GROUP, cmd: SESSION_PROTECTION_CMD, listed: &[1, 2] },
    // iwl_mac_config_cmd, MAC_CONTEXT_CONFIG_CMD_API_S_VER_2.
    Api { group: MAC_CONF_GROUP, cmd: MAC_CONFIG_CMD, listed: &[2] },
    // iwl_link_config_cmd, LINK_CONTEXT_CONFIG_CMD_API_S_VER_1.
    Api { group: MAC_CONF_GROUP, cmd: LINK_CONFIG_CMD, listed: &[] },
    // iwl_sta_cfg_cmd_v1, STA_CMD_API_S_VER_1.
    Api { group: MAC_CONF_GROUP, cmd: STA_CONFIG_CMD, listed: &[] },
    // iwl_remove_sta_cmd, version 1.
    Api { group: MAC_CONF_GROUP, cmd: STA_REMOVE_CMD, listed: &[] },
    // iwl_rlc_config_cmd, RLC_CONFIG_CMD_API_S_VER_2.
    Api { group: DATA_PATH_GROUP, cmd: RLC_CONFIG_CMD, listed: &[2] },
    // iwl_scd_queue_cfg_cmd, TX_QUEUE_CFG_CMD_API_S_VER_3.
    Api { group: DATA_PATH_GROUP, cmd: SCD_QUEUE_CONFIG_CMD, listed: &[3] },
    // iwl_sec_key_cmd, SEC_KEY_CMD_API_S_VER_1.
    Api { group: DATA_PATH_GROUP, cmd: SEC_KEY_CMD, listed: &[1] },
];

/// The first command whose reported version this driver does not encode,
/// as `(group, cmd)`, or `Ok` when the firmware speaks every layout here.
pub fn check(ucode: &Ucode<'_>) -> Result<(), (u8, u8)> {
    for a in &APIS {
        let ok = match ucode.cmd_version(a.group, a.cmd) {
            Some(v) if a.listed.is_empty() => v == 1,
            Some(v) => a.listed.contains(&v),
            None => a.listed.is_empty(),
        };
        if !ok {
            return Err((a.group, a.cmd));
        }
    }
    Ok(())
}
