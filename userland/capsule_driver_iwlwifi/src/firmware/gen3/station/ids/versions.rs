// NONOS Operating System (AGPL-3.0-or-later)

//! The layout version of each command this driver encodes, and the check that the firmware runs it.

use super::super::super::cmds::{LINK_CONFIG_CMD, LONG_GROUP, MAC_CONFIG_CMD, MAC_CONF_GROUP};
use super::super::super::ucode::Ucode;
use super::opcodes::{
    DATA_PATH_GROUP, MAC_PM_POWER_TABLE, PHY_CONTEXT_CMD, RLC_CONFIG_CMD, SCAN_ABORT_UMAC,
    SCD_QUEUE_CONFIG_CMD, SEC_KEY_CMD, SESSION_PROTECTION_CMD, STA_CONFIG_CMD, STA_REMOVE_CMD,
    TXPATH_FLUSH, TX_CMD,
};

/// One command and the versions whose layout this driver encodes. An empty
/// `listed` means the firmware must not list the command at all (or list it
/// as version 1): Linux then sends the version 1 layout.
pub struct Api {
    pub group: u8,
    pub cmd: u8,
    pub listed: &'static [u8],
}

/// Every command the join sends.
pub const APIS: [Api; 13] = [
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
    // iwl_mac_power_cmd, MAC_PM_POWER_TABLE version 1.
    Api { group: LONG_GROUP, cmd: MAC_PM_POWER_TABLE, listed: &[1] },
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
