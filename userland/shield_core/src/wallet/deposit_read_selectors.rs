//! The pool's view selectors a deposit reads, from the compiled pool's method table.

/// Selectors, from the compiled pool's method table.
pub(super) const DEPOSITS_PAUSED: [u8; 4] = [0x60, 0xda, 0x3e, 0x83];
pub(super) const WOUND_DOWN: [u8; 4] = [0x1c, 0x39, 0x6d, 0xa3];
pub(super) const BETA_MODE: [u8; 4] = [0x8a, 0xbe, 0x2f, 0x26];
pub(super) const BETA_PAUSED: [u8; 4] = [0x8a, 0xe0, 0xb0, 0x58];
pub(super) const BETA_DEPOSITOR: [u8; 4] = [0x51, 0xcc, 0x9f, 0x48];
pub(super) const BETA_DEPOSITED: [u8; 4] = [0x58, 0xdb, 0x32, 0x5c];
pub(super) const BETA_ADDR_CAP: [u8; 4] = [0x23, 0x38, 0x53, 0xbf];
pub(super) const BETA_TOTAL_DEPOSITED: [u8; 4] = [0x83, 0xf9, 0x56, 0xe2];
pub(super) const BETA_TOTAL_CAP: [u8; 4] = [0xf3, 0x0e, 0x5e, 0x0f];
pub(super) const SHIELD_FEE_BPS: [u8; 4] = [0x54, 0x12, 0x52, 0xf9];
/// Launch shape only: `scale(uint64)` and `openDeposits()`.
pub(super) const SCALE: [u8; 4] = [0x1e, 0x89, 0xff, 0x8f];
pub(super) const OPEN_DEPOSITS: [u8; 4] = [0xc3, 0xf8, 0x1c, 0xc5];
