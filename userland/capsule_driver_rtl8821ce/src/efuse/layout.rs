// NONOS Operating System (AGPL-3.0-or-later)

//! The efuse registers, their fields, and the logical map's offsets.

/// The efuse control register: address in bits 8..18, data in the low byte, and
/// bit 31 the read-complete flag.
pub(super) const REG_EFUSE_CTRL: usize = 0x0030;
/// The LDO/efuse control register: its bank-select bits and, at byte 3, the 2.5V
/// LDO enable.
pub(super) const REG_LDO_EFUSE_CTRL: usize = 0x0034;
/// The chip configuration register whose bits 12..16 carry the cut version.
pub(super) const REG_CHIP_VER: usize = 0x00F0;

pub(super) const EF_ADDR_MASK: u32 = 0x3ff;
pub(super) const EF_ADDR_SHIFT: u32 = 8;
pub(super) const EF_DATA_MASK: u32 = 0xff;
pub(super) const EF_FLAG: u32 = 1 << 31;
/// Bank select bits 8..9 of the LDO/efuse control; zero selects the wifi bank.
pub(super) const BANK_SEL_MASK: u32 = (1 << 8) | (1 << 9);
/// Bit 7 of `REG_LDO_EFUSE_CTRL + 3`: the 2.5V LDO enable, cleared for the read.
pub(super) const LDO25_EN: u8 = 1 << 7;

/// The physical and logical efuse are both 512 bytes on the 8821c.
pub(super) const EFUSE_SIZE: usize = 512;
/// The logical offset of the RF front-end option byte.
pub(super) const RFE_OPTION_OFFSET: usize = 0xCA;
