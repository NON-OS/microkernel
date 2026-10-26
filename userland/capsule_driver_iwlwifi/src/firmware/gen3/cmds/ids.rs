// NONOS Operating System (AGPL-3.0-or-later)

//! Command groups, command and notification ids, and the capability and API bits the bring-up reads.

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
