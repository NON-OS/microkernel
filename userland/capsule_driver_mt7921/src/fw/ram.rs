/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The RAM code file's layout, `struct mt76_connac2_fw_trailer` and
//! `struct mt76_connac2_fw_region` of Linux v6.12 `mt76_connac_mcu.h`,
//! little-endian: a 36-byte trailer at the end counts the regions (byte
//! 2), whose 40-byte records sit just before it; the regions' bytes run
//! back to back from the start of the file, in record order
//! (`mt76_connac_mcu_send_ram_firmware`). Every region must end before the
//! records begin; Linux trusts the file for that.

use alloc::vec::Vec;

const TRAILER: usize = 36;
const REGION: usize = 40;
/// The most regions a file may list; the shipped ones have five or six.
pub const MAX_REGIONS: usize = 16;

/// `FW_FEATURE_NON_DL`: a region the chip is not sent (the CLC tables).
pub const FEATURE_NON_DL: u8 = 1 << 6;
/// `FW_FEATURE_OVERRIDE_ADDR`: the region whose address the firmware
/// starts at.
pub const FEATURE_OVERRIDE_ADDR: u8 = 1 << 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region {
    pub offs: usize,
    pub addr: u32,
    pub len: usize,
    pub feature: u8,
}

/// The regions of a RAM image, or `None` for one not shaped so.
pub fn regions(img: &[u8]) -> Option<Vec<Region>> {
    let t = img.len().checked_sub(TRAILER)?;
    let n = img[t + 2] as usize;
    if n == 0 || n > MAX_REGIONS {
        return None;
    }
    let table = t.checked_sub(n * REGION)?;
    let mut offs = 0usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let r = &img[table + i * REGION..table + (i + 1) * REGION];
        let word = |at: usize| u32::from_le_bytes([r[at], r[at + 1], r[at + 2], r[at + 3]]);
        let (addr, len, feature) = (word(16), word(20) as usize, r[24]);
        if offs.checked_add(len)? > table {
            return None;
        }
        out.push(Region { offs, addr, len, feature });
        offs += len;
    }
    Some(out)
}
