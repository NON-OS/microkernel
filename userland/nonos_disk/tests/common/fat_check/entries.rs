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

//! A directory's 32-byte slots, up to the zero slot that ends it: each short
//! slot with the long name its VFAT slots spell, held to the rules a driver
//! checks before it believes a long name. Ordinals count down to one, the
//! first is flagged last, every slot carries the short name's checksum, the
//! name ends in one zero and 0xFFFF padding, and there is no spare slot.

pub struct Entry {
    pub name: String,
    pub short: [u8; 11],
    pub attr: u8,
    pub cluster: u32,
    pub size: u32,
}

pub const ATTR_DIR: u8 = 0x10;
const ATTR_LONG: u8 = 0x0F;

/// The checksum of a short name, as the specification computes it.
pub fn checksum(short: &[u8; 11]) -> u8 {
    short.iter().fold(0u8, |sum, &c| ((sum & 1) << 7).wrapping_add(sum >> 1).wrapping_add(c))
}

pub fn parse(bytes: &[u8]) -> Vec<Entry> {
    let mut out = Vec::new();
    let mut pending: Vec<&[u8]> = Vec::new();
    for slot in bytes.chunks_exact(32) {
        if slot[0] == 0 {
            assert!(pending.is_empty(), "long-name slots with no short slot after them");
            return out;
        }
        assert_ne!(slot[0], 0xE5, "a deleted slot in a volume written once");
        if slot[11] == ATTR_LONG {
            pending.push(slot);
            continue;
        }
        let short: [u8; 11] = slot[..11].try_into().unwrap();
        let long = (!pending.is_empty()).then(|| join(&pending, &short));
        pending.clear();
        let hi = u16::from_le_bytes([slot[20], slot[21]]) as u32;
        let lo = u16::from_le_bytes([slot[26], slot[27]]) as u32;
        out.push(Entry {
            name: long.unwrap_or_else(|| short_text(&short, slot[12])),
            short,
            attr: slot[11],
            cluster: hi << 16 | lo,
            size: u32::from_le_bytes([slot[28], slot[29], slot[30], slot[31]]),
        });
    }
    panic!("a directory with no zero slot to end it");
}

fn join(slots: &[&[u8]], short: &[u8; 11]) -> String {
    let (n, sum) = (slots.len(), checksum(short));
    for (i, s) in slots.iter().enumerate() {
        let flag = if i == 0 { 0x40 } else { 0 };
        assert_eq!(s[0], (n - i) as u8 | flag, "long-name ordinal");
        assert_eq!((s[12], s[13], s[26], s[27]), (0, sum, 0, 0), "long-name type, checksum");
    }
    let units: Vec<u16> = slots
        .iter()
        .rev()
        .flat_map(|s| {
            let at = [1, 3, 5, 7, 9, 14, 16, 18, 20, 22, 24, 28, 30];
            at.map(|o| u16::from_le_bytes([s[o], s[o + 1]]))
        })
        .collect();
    let end = units.iter().position(|&u| u == 0).unwrap_or(units.len());
    assert!(units[end..].iter().skip(1).all(|&u| u == 0xFFFF), "padding after the name");
    assert_eq!(n, end.div_ceil(13), "a long name in more slots than it needs");
    String::from_utf16(&units[..end]).expect("a long name in UCS-2")
}

/// The 8.3 name as a listing shows it, lower case where the reserved
/// byte's flags say the part was written so.
fn short_text(short: &[u8; 11], nt: u8) -> String {
    let part = |b: &[u8], lower: bool| {
        let t = String::from_utf8_lossy(b).trim_end().to_string();
        if lower {
            t.to_ascii_lowercase()
        } else {
            t
        }
    };
    let (base, ext) = (part(&short[..8], nt & 0x08 != 0), part(&short[8..], nt & 0x10 != 0));
    if ext.is_empty() {
        base
    } else {
        format!("{base}.{ext}")
    }
}
