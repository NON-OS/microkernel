/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The ROM patch file's layout, `struct mt76_connac2_patch_hdr` and
//! `struct mt76_connac2_patch_sec` of Linux v6.12 `mt76_connac_mcu.h`, all
//! big-endian: a 96-byte header whose descriptor counts the sections at
//! byte 44, then one 64-byte section record each. A section must be of
//! the information type (`PATCH_SEC_TYPE_INFO`), and its bytes must lie in
//! the file; Linux checks the type and trusts the rest.

use alloc::vec::Vec;

const HDR: usize = 96;
const SEC: usize = 64;
const N_REGION_AT: usize = 44;
const TYPE_MASK: u32 = 0xffff;
const TYPE_INFO: u32 = 0x2;
/// The most sections a file may list; every one shipped has one.
pub const MAX_SECTIONS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Section {
    /// Where the bytes start in the file.
    pub offs: usize,
    /// Where they go on the chip, and how many.
    pub addr: u32,
    pub len: usize,
    /// `sec_key_idx`: how the bytes are encrypted (`mode::patch_mode`).
    pub info: u32,
}

fn be(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

/// The sections of a patch image, or `None` for one that is not shaped
/// as the layout says.
pub fn sections(img: &[u8]) -> Option<Vec<Section>> {
    let n = be(img, N_REGION_AT)? as usize;
    if n == 0 || n > MAX_SECTIONS {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let at = HDR + i * SEC;
        if be(img, at)? & TYPE_MASK != TYPE_INFO {
            return None;
        }
        let offs = be(img, at + 4)? as usize;
        let (addr, len, info) = (be(img, at + 12)?, be(img, at + 16)? as usize, be(img, at + 20)?);
        if len == 0 || offs.checked_add(len)? > img.len() {
            return None;
        }
        out.push(Section { offs, addr, len, info });
    }
    Some(out)
}
