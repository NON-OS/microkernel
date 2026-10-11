// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! IDENTIFY DEVICE (ACS-3, 7.12.7) read for what the driver needs: the
//! capacity and whether it takes LBA48, that it does DMA, how to flush its
//! cache, and its model and serial. Pure, for the host proofs.

use crate::constants::ata::{IDENTIFY_WORDS, LBA28_LIMIT, LBA48_LIMIT, SECTOR_BYTES};

/// How the disk's write cache is flushed, if it has one to flush.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flush {
    /// No write cache: a write is on the medium when it completes.
    None,
    Cache,
    CacheExt,
}

#[derive(Clone, Copy)]
pub struct Identity {
    pub sectors: u64,
    pub lba48: bool,
    pub flush: Flush,
    /// As the device sends them: ASCII, space padded.
    pub model: [u8; 40],
    pub serial: [u8; 20],
}

const W_CONFIG: usize = 0;
const W_SERIAL: usize = 10;
const W_MODEL: usize = 27;
const W_CAPS: usize = 49;
const W_LBA28: usize = 60;
const W_CMDSET_SUP2: usize = 83;
const W_CMDSET_SUP1: usize = 82;
const W_LBA48: usize = 100;
const W_SECTOR_SIZE: usize = 106;
const W_LOGICAL_SIZE: usize = 117;

/// Word 0 bit 15 set: not an ATA device.
const CONFIG_NOT_ATA: u16 = 1 << 15;
const CAPS_DMA: u16 = 1 << 8;
const CAPS_LBA: u16 = 1 << 9;
const SUP1_WRITE_CACHE: u16 = 1 << 5;
const SUP2_LBA48: u16 = 1 << 10;
const SUP2_FLUSH: u16 = 1 << 12;
const SUP2_FLUSH_EXT: u16 = 1 << 13;
/// Words 82..=83 and 106 are valid when bit 14 is set and bit 15 clear.
const VALID_MASK: u16 = 0xC000;
const VALID: u16 = 0x4000;
const SIZE_LONG_LOGICAL: u16 = 1 << 12;

pub fn parse(w: &[u16; IDENTIFY_WORDS]) -> Result<Identity, &'static str> {
    if w[W_CONFIG] & CONFIG_NOT_ATA != 0 {
        return Err("not an ATA disk");
    }
    if w[W_CAPS] & CAPS_LBA == 0 {
        return Err("no LBA addressing");
    }
    if w[W_CAPS] & CAPS_DMA == 0 {
        return Err("no DMA");
    }
    let valid = |word: u16| word & VALID_MASK == VALID;
    let sup1 = if valid(w[W_CMDSET_SUP1]) { w[W_CMDSET_SUP1] } else { 0 };
    let sup2 = if valid(w[W_CMDSET_SUP2]) { w[W_CMDSET_SUP2] } else { 0 };
    let size = w[W_SECTOR_SIZE];
    if valid(size) && size & SIZE_LONG_LOGICAL != 0 {
        let words = u32::from(w[W_LOGICAL_SIZE]) | u32::from(w[W_LOGICAL_SIZE + 1]) << 16;
        if words as usize * 2 != SECTOR_BYTES {
            return Err("logical sectors are not 512 bytes");
        }
    }
    let lba28 = u64::from(w[W_LBA28]) | u64::from(w[W_LBA28 + 1]) << 16;
    let lba48 = sup2 & SUP2_LBA48 != 0;
    let sectors = if lba48 {
        (0..4).fold(0u64, |acc, i| acc | u64::from(w[W_LBA48 + i]) << (16 * i))
    } else {
        lba28
    };
    let limit = if lba48 { LBA48_LIMIT } else { LBA28_LIMIT };
    if sectors == 0 || sectors > limit {
        return Err("no usable capacity");
    }
    let flush = if lba48 && sup2 & SUP2_FLUSH_EXT != 0 {
        Flush::CacheExt
    } else if sup2 & SUP2_FLUSH != 0 || sup1 & SUP1_WRITE_CACHE != 0 {
        Flush::Cache
    } else {
        Flush::None
    };
    Ok(Identity {
        sectors,
        lba48,
        flush,
        model: text(&w[W_MODEL..W_MODEL + 20]),
        serial: text(&w[W_SERIAL..W_SERIAL + 10]),
    })
}

/// An ATA string: two characters a word, the first in the high byte.
fn text<const N: usize>(words: &[u16]) -> [u8; N] {
    core::array::from_fn(|i| {
        let w = words[i / 2];
        if i % 2 == 0 {
            (w >> 8) as u8
        } else {
            w as u8
        }
    })
}
