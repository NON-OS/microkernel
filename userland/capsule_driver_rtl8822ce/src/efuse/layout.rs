// NONOS Operating System (AGPL-3.0-or-later)

//! The efuse registers, the 8822C's map sizes and the offsets read from it:
//! rtw88 `reg.h`, `rtw8822c_hw_spec` (phy 512, log 768, protect 124) and
//! `struct rtw8822c_efuse` / the `PPG_*` offsets in `rtw8822c.h`.

/// Address in bits 8..17, data in the low byte, bit 31 the read-complete flag.
pub(super) const REG_EFUSE_CTRL: usize = 0x0030;
/// The bank-select bits 8..9; zero selects the wifi bank.
pub(super) const REG_LDO_EFUSE_CTRL: usize = 0x0034;
/// `REG_SYS_CFG1`, whose bits 12..15 carry the cut version.
pub(super) const REG_CHIP_VER: usize = 0x00F0;
/// `REG_ANAPARLDO_POW_MAC` and `BIT_LDOE25_PON`: the 8822C's 2.5V LDO, off
/// for the read (`rtw8822c_cfg_ldo25`), where the 8821C uses 0x37 bit 7.
pub(super) const REG_ANAPARLDO_POW_MAC: usize = 0x0029;
pub(super) const LDOE25_PON: u8 = 1 << 0;

pub(super) const EF_ADDR_MASK: u32 = 0x3ff;
pub(super) const EF_ADDR_SHIFT: u32 = 8;
pub(super) const EF_DATA_MASK: u32 = 0xff;
pub(super) const EF_FLAG: u32 = 1 << 31;
pub(super) const BANK_SEL_MASK: u32 = (1 << 8) | (1 << 9);

/// The physical efuse, the bytes the logical map may be packed in (the rest
/// is the protected area, where the trim values are), and the logical map.
pub(super) const PHYS_SIZE: usize = 512;
pub(super) const PACKED_SIZE: usize = PHYS_SIZE - 124;
pub(super) const LOG_SIZE: usize = 768;

// The logical map (struct rtw8822c_efuse).
pub(super) const TXPWR_IDX: usize = 0x10;
/// Each path's struct rtw_txpwr_idx: 18 bytes of 2G, then 24 of 5G.
pub(super) const TXPWR_STRIDE: usize = 42;
pub(super) const XTAL_K: usize = 0xB9;
pub(super) const RFE_OPTION: usize = 0xCA;

// The physical map's protected area (PPG_*, rtw8822c.h).
pub(super) const PPG_2G_TXAB: [usize; 3] = [0x1D4, 0x1EE, 0x1D2]; // L, M, H
pub(super) const PPG_5G_TX: [[usize; 5]; 2] =
    [[0x1EC, 0x1E8, 0x1E4, 0x1E0, 0x1DC], [0x1EB, 0x1E7, 0x1E3, 0x1DF, 0x1DB]];
pub(super) const PPG_THERMAL: [usize; 2] = [0x1EF, 0x1B0];
pub(super) const PPG_PABIAS_2G: [usize; 2] = [0x1D6, 0x1D5];
pub(super) const PPG_PABIAS_5G: [usize; 2] = [0x1D8, 0x1D7];
