/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The WFDMA descriptor, `struct mt76_desc` of Linux v6.12 `mt76/dma.h`:
//! four little-endian words, `buf0`, `ctrl`, `buf1`, `info`. The bits are
//! the `MT_DMA_CTL_*` facts of the same header.

pub const DESC_SIZE: usize = 16;
pub const CTL_SD_LEN0_SHIFT: u32 = 16;
pub const CTL_SD_LEN0: u32 = 0x3fff << CTL_SD_LEN0_SHIFT;
pub const CTL_LAST_SEC0: u32 = 1 << 30;
pub const CTL_DMA_DONE: u32 = 1 << 31;
pub const CTL_TO_HOST_A: u32 = 1 << 12;
pub const CTL_DROP: u32 = 1 << 14;
/// The later descriptor layout: its drop reason is in `buf1` bits 15:12
/// (`MT_DMA_WED_IND_REASON`: 1 a repeat, 2 an old frame) and the PN check.
pub const CTL_VER_MASK: u32 = 1 << 7;
pub const CTL_PN_CHK_FAIL: u32 = 1 << 13;
pub const INFO_DMA_FRAG: u32 = 1 << 9;

/// One descriptor of a host ring, as offsets into its array.
pub const BUF0: usize = 0;
pub const CTRL: usize = 4;
pub const BUF1: usize = 8;
pub const INFO: usize = 12;

/// The control word of a single-segment buffer of `len` bytes. A length past
/// the field's fourteen bits is cut to it, never spilled into the flags.
pub fn ctrl_one(len: usize) -> u32 {
    let len = (len.min(0x3fff) as u32) << CTL_SD_LEN0_SHIFT;
    len | CTL_LAST_SEC0
}

/// What a completed receive descriptor (`ctrl`, `buf1`, `info`) says: the
/// bytes written, whether the frame goes on in the next one, and whether
/// the DMA asked it be dropped, as `mt76_dma_should_drop_buf` decides.
pub fn rx_done(ctrl: u32, buf1: u32, info: u32) -> (usize, bool, bool) {
    let len = ((ctrl & CTL_SD_LEN0) >> CTL_SD_LEN0_SHIFT) as usize;
    let more = ctrl & CTL_LAST_SEC0 == 0;
    let drop = match (ctrl & CTL_VER_MASK != 0, buf1 >> 12 & 0xf) {
        (false, _) => ctrl & (CTL_TO_HOST_A | CTL_DROP) != 0,
        (true, 1) => true,
        (true, 2) => info & INFO_DMA_FRAG == 0,
        (true, _) => ctrl & CTL_PN_CHK_FAIL != 0,
    };
    (len, more, drop)
}
