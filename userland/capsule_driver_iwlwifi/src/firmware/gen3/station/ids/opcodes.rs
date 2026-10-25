// NONOS Operating System (AGPL-3.0-or-later)

//! The groups and opcodes of the join's commands.

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
