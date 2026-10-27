// NONOS Operating System (AGPL-3.0-or-later)

//! The discrete AX210 (TY, 8086:2725) and Meteor Lake (MA, 8086:7E40 and
//! 2729) on the gen3 path, host-proven only: the real linux-firmware images
//! (API 86, linux-firmware bf992df7) parse with the driver's own parser and
//! speak every version the driver encodes, their PNVM files give each its
//! own section, the discrete AX210 boots with its boot LTR, and a dual-radio
//! (GF4) module puts a 5 GHz PHY on LMAC 1. No TY or MA part has booted it.

mod boot;
mod carried;
mod differs;
mod firmware;
mod rig;

use crate::gen3::cmds::*;

/// What the bring-up and the scan send or read, beside the join's `APIS`.
pub(crate) const SPOKEN: &[(u8, u8)] = &[
    (SYSTEM_GROUP, INIT_EXTENDED_CFG_CMD),
    (REGULATORY_AND_NVM_GROUP, NVM_ACCESS_COMPLETE),
    (REGULATORY_AND_NVM_GROUP, NVM_GET_INFO),
    (REGULATORY_AND_NVM_GROUP, PNVM_INIT_COMPLETE_NTFY),
    (SYSTEM_GROUP, SOC_CONFIGURATION_CMD),
    (LONG_GROUP, TX_ANT_CONFIGURATION_CMD),
    (LONG_GROUP, BT_CONFIG),
    (LONG_GROUP, POWER_TABLE_CMD),
    (LONG_GROUP, MCC_UPDATE_CMD),
    (LONG_GROUP, SCAN_CFG_CMD),
    (LONG_GROUP, SCAN_REQ_UMAC),
    (LONG_GROUP, SCAN_COMPLETE_UMAC),
    (MAC_CONF_GROUP, MAC_CONFIG_CMD),
    (MAC_CONF_GROUP, LINK_CONFIG_CMD),
    (MAC_CONF_GROUP, 0xF6),
    (MAC_CONF_GROUP, 0xFB),
    (LEGACY_GROUP, 0x01),
    (LEGACY_GROUP, REPLY_RX_MPDU_CMD),
    (LEGACY_GROUP, 0x1C),
];
