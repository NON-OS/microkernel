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

//! A long name, for a path component no 8.3 name spells: the VFAT slots that
//! carry it in UCS-2, thirteen characters each, last part first, then the
//! short slot they belong to under a generated alias. A driver that reads
//! long names, UEFI's among them, finds the file by the name given; the
//! alias is all one that does not would see. The loader opens
//! `bootloader.trailer` and `boot_root.approval` by those names, and no 8.3
//! name holds either.

use alloc::format;
use alloc::vec::Vec;

use super::part::allowed;
use super::short_name::NameError;

/// Attribute byte of a long-name slot: read-only, hidden, system, volume.
pub const ATTR_LONG_NAME: u8 = 0x0F;
/// The flag on the ordinal of the slot that holds a name's last part.
const LAST: u8 = 0x40;
/// UCS-2 characters one slot holds.
const PER_SLOT: usize = 13;
/// The longest long name, in UCS-2 characters.
const MAX: usize = 255;

/// The name in UCS-2, or why it cannot be a long name.
pub fn units(name: &str) -> Result<Vec<u16>, NameError> {
    if name.is_empty() {
        return Err(NameError::Empty);
    }
    if name.ends_with(['.', ' ']) || name.chars().any(|c| c < ' ' || "\"*/:<>?\\|".contains(c)) {
        return Err(NameError::BadChar);
    }
    let units: Vec<u16> = name.encode_utf16().collect();
    if units.len() > MAX {
        return Err(NameError::TooLong);
    }
    Ok(units)
}

/// Slots a long name of `units` characters takes before its short slot.
pub fn slots(units: usize) -> usize {
    units.div_ceil(PER_SLOT)
}

/// The checksum of the short name every one of its long slots carries.
pub fn checksum(short: &[u8; 11]) -> u8 {
    short.iter().fold(0u8, |sum, &c| sum.rotate_right(1).wrapping_add(c))
}

/// The long slots for `units`, in the order they are written: the slot with
/// the last part first, flagged, down to the one with the first. A name that
/// does not fill its last slot ends in a zero and is padded with 0xFFFF.
pub fn encode(units: &[u16], short: &[u8; 11]) -> Vec<[u8; 32]> {
    let sum = checksum(short);
    let n = slots(units.len());
    let mut out = Vec::with_capacity(n);
    for ord in (1..=n).rev() {
        let mut part = [0xFFFFu16; PER_SLOT];
        for (i, c) in part.iter_mut().enumerate() {
            match units.get((ord - 1) * PER_SLOT + i) {
                Some(&u) => *c = u,
                None => {
                    *c = 0;
                    break;
                }
            }
        }
        let mut s = [0u8; 32];
        s[0] = ord as u8 | if ord == n { LAST } else { 0 };
        put(&mut s, 1, &part[..5]);
        s[11] = ATTR_LONG_NAME;
        s[13] = sum;
        put(&mut s, 14, &part[5..11]);
        put(&mut s, 28, &part[11..]);
        out.push(s);
    }
    out
}

fn put(slot: &mut [u8; 32], at: usize, chars: &[u16]) {
    for (i, c) in chars.iter().enumerate() {
        slot[at + 2 * i..at + 2 * i + 2].copy_from_slice(&c.to_le_bytes());
    }
}

/// The 8.3 alias for a long name, unlike any in `taken`: the base's first
/// characters, `~` and a number, then up to three of the extension, upper
/// case, with `_` for any character a short name cannot hold.
pub fn alias(name: &str, taken: &[[u8; 11]]) -> [u8; 11] {
    let (base, ext) = match name.rfind('.') {
        Some(i) if i > 0 => (&name[..i], &name[i + 1..]),
        _ => (name, ""),
    };
    let short = |s: &str| -> Vec<u8> {
        s.bytes()
            .filter(|&c| c != b' ' && c != b'.')
            .map(|c| if allowed(c) { c.to_ascii_uppercase() } else { b'_' })
            .collect()
    };
    let (base, ext) = (short(base), short(ext));
    let mut out = [b' '; 11];
    for (o, &c) in out[8..].iter_mut().zip(ext.iter()) {
        *o = c;
    }
    let mut n = 1u32;
    loop {
        let tail = format!("~{n}");
        let keep = base.len().min(8 - tail.len());
        out[..8].fill(b' ');
        out[..keep].copy_from_slice(&base[..keep]);
        out[keep..keep + tail.len()].copy_from_slice(tail.as_bytes());
        if !taken.contains(&out) {
            return out;
        }
        n += 1;
    }
}
