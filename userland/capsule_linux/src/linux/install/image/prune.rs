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

//! Which images the cache drops: it is held to a budget by dropping the
//! least recently used image first, never one whose app is installed, and
//! never the one just used. An installed app's image stays whatever the
//! budget, so a re-install of it is always a cache hit; the cache may then
//! stay over its budget until one is removed. Pure, so the host proofs hold
//! it.

use alloc::vec::Vec;

/// One image in the cache.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cached {
    pub pin: [u8; 32],
    /// What its directory holds.
    pub bytes: u64,
    /// When an install last used it, in ms; 0 when it never said.
    pub used: u64,
    /// Its app is installed.
    pub installed: bool,
}

/// The pins to drop so the rest fit `budget`, oldest use first.
pub fn evict(cache: &[Cached], budget: u64, keep: &[u8; 32]) -> Vec<[u8; 32]> {
    let mut total: u64 = cache.iter().map(|c| c.bytes).fold(0, u64::saturating_add);
    let mut droppable: Vec<&Cached> = cache.iter().filter(|c| !c.installed && c.pin != *keep).collect();
    droppable.sort_by(|a, b| a.used.cmp(&b.used).then(a.pin.cmp(&b.pin)));
    let mut out = Vec::new();
    for c in droppable {
        if total <= budget {
            break;
        }
        total = total.saturating_sub(c.bytes);
        out.push(c.pin);
    }
    out
}

/// The `used` stamp as kept: eight bytes, little-endian ms.
pub fn stamp(ms: u64) -> [u8; 8] {
    ms.to_le_bytes()
}

/// A kept stamp read back; 0 for anything that is not one.
pub fn read_stamp(raw: &[u8]) -> u64 {
    <[u8; 8]>::try_from(raw).map_or(0, u64::from_le_bytes)
}

/// The cache budget in bytes from `/etc/nonos-image-cache-max`'s first word,
/// in MiB, within its floor and ceiling; the default for anything else.
pub fn budget(setting: Option<&[u8]>) -> u64 {
    const DEFAULT_MIB: u64 = 2048;
    const FLOOR_MIB: u64 = 256;
    const CEILING_MIB: u64 = 64 << 10;
    let word = setting.and_then(|s| s.split(|b| b.is_ascii_whitespace()).find(|w| !w.is_empty()));
    let mib = word.and_then(|w| core::str::from_utf8(w).ok()?.parse::<u64>().ok());
    mib.map_or(DEFAULT_MIB, |m| m.clamp(FLOOR_MIB, CEILING_MIB)) << 20
}
