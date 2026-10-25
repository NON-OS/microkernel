// NONOS Operating System (AGPL-3.0-or-later)

//! The DDMA channel 0 registers and bits.

// The firmware-download control register lives with the rest of the download
// register map; re-exported here so the DDMA proofs keep importing it alongside
// the channel definitions.
pub use super::super::regs::{
    DMEM_CHKSUM_OK, DMEM_DW_OK, IMEM_CHKSUM_OK, IMEM_DW_OK, MCUFWDL_EN, OCPBASE_DMEM,
    REG_MCUFW_CTRL,
};

// DDMA channel-0 registers. These are the rtw88 8821c DDMA channel semantics
// (`reg.h`): OWN in bit 31, checksum-enable in bit 29, checksum-status in bit 27,
// reset-checksum in bit 25, checksum-continue in bit 24, and an 18-bit transfer
// length. Verified against rtw88 `reg.h` (BIT_DDMACH0_*): these values are exact.
pub const REG_DDMA_CH0SA: usize = 0x1200;
pub const REG_DDMA_CH0DA: usize = 0x1204;
pub const REG_DDMA_CH0CTRL: usize = 0x1208;

// Control-word bits.
pub const DDMA_OWN: u32 = 1 << 31;
pub const DDMA_CHKSUM_EN: u32 = 1 << 29;
pub const DDMA_CHKSUM_STS: u32 = 1 << 27;
pub const DDMA_RESET_CHKSUM_STS: u32 = 1 << 25;
pub const DDMA_CHKSUM_CONT: u32 = 1 << 24;
pub const DDMA_DLEN_MASK: u32 = 0x3FFFF;

/// How long a DDMA transfer may hold the channel (rtw88 `check_hw_ready`).
pub(super) const OWN_MS: u64 = 10;
