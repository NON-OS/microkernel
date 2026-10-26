// NONOS Operating System (AGPL-3.0-or-later)

//! The RX descriptor's fields, at rtw88's `rx.h` `RTW_RX_DESC_W*` positions.

// Word 0 fields.
pub(super) const W0_PKT_LEN: u32 = 0x0000_3FFF; // GENMASK(13, 0)
pub(super) const W0_CRC32: u32 = 1 << 14;
pub(super) const W0_ICV_ERR: u32 = 1 << 15;
pub(super) const W0_DRV_INFO_SHIFT: u32 = 16; // GENMASK(19, 16), in 8-byte units
pub(super) const W0_DRV_INFO_MASK: u32 = 0xF;
pub(super) const W0_ENC_TYPE_SHIFT: u32 = 20; // GENMASK(22, 20)
pub(super) const W0_ENC_TYPE_MASK: u32 = 0x7;
pub(super) const W0_SHIFT_SHIFT: u32 = 24; // GENMASK(25, 24), in bytes
pub(super) const W0_SHIFT_MASK: u32 = 0x3;
pub(super) const W0_PHYST: u32 = 1 << 26;
pub(super) const W0_SWDEC: u32 = 1 << 27;
// Word 2 fields.
pub(super) const W2_C2H: u32 = 1 << 28;
