/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The patch file's layout, as Linux v6.12 `btmtk_setup_firmware_79xx`
//! reads it. A 32-octet header (build time, platform, hardware and
//! software version), a 64-octet global descriptor whose fourth word is
//! the number of sections, then a 64-octet map for each: its type, its
//! offset in the file and its size, then 52 octets the chip is given whole
//! when asked whether it holds the section, which begin with its load
//! address and the octets to download. A section of no octets is not
//! sent. Every map and every section's octets must lie within the file.

use alloc::vec::Vec;

const HEADER: usize = 32;
const GLOBAL: usize = 64;
const MAP: usize = 64;
const COMMON: usize = 12;
pub const SPEC: usize = MAP - COMMON;
/// More sections than a patch carries (the MT7921's has four); a count
/// past it is a broken header.
const MOST: usize = 16;

pub struct Section {
    pub spec: [u8; SPEC],
    pub at: usize,
    pub len: usize,
}

fn le32(b: &[u8], at: usize) -> usize {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]) as usize
}

pub fn sections(img: &[u8]) -> Option<Vec<Section>> {
    let count = le32(img.get(..HEADER + GLOBAL)?, HEADER + 12);
    if count > MOST {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let map = img.get(HEADER + GLOBAL + MAP * i..)?.get(..MAP)?;
        let (at, len) = (le32(map, 4), le32(map, COMMON + 4));
        if len > 0 && at.checked_add(len).is_none_or(|end| end > img.len()) {
            return None;
        }
        let mut spec = [0u8; SPEC];
        spec.copy_from_slice(&map[COMMON..]);
        out.push(Section { spec, at, len });
    }
    Some(out)
}
