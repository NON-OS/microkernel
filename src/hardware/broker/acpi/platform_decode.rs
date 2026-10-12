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

//! Decoding a platform device record (`platform_wire` lays it out). Pure:
//! `kernel_proofs` mounts this file as it is.

use super::platform_wire::{Platform, DEPS_UNMET, GPIO, I2C, MAX_WINDOWS, SD};
use alloc::vec::Vec;

fn u64_at(b: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(b[i..i + 8].try_into().unwrap_or([0; 8]))
}

pub fn decode(b: &[u8]) -> Option<Platform> {
    let h = b.get(..32)?;
    let (version, class, n, irq_flags, flags) = (h[0], h[1], h[2] as usize, h[3], h[4]);
    let zeros = h[5..8].iter().chain(&h[28..32]).all(|x| *x == 0) && flags & !DEPS_UNMET == 0;
    if version != 1
        || !matches!(class, I2C | SD | GPIO)
        || !(1..=MAX_WINDOWS).contains(&n)
        || !zeros
    {
        return None;
    }
    let mut hid = [0u8; 8];
    hid.copy_from_slice(&h[8..16]);
    let named = hid.iter().take_while(|c| **c != 0).count();
    if named == 0
        || !hid[..named].iter().all(|c| c.is_ascii_alphanumeric())
        || hid[named..].iter().any(|c| *c != 0)
    {
        return None;
    }
    let w = b.get(32..32 + n * 16)?;
    let windows: Vec<(u64, u64)> =
        w.chunks_exact(16).map(|c| (u64_at(c, 0), u64_at(c, 8))).collect();
    let len = *b.get(32 + n * 16)? as usize;
    let path = b.get(33 + n * 16..33 + n * 16 + len)?;
    let printable = path.first() == Some(&b'\\') && path.iter().all(|c| c.is_ascii_graphic());
    if len == 0 || len > 127 || !printable || b.len() != 33 + n * 16 + len {
        return None;
    }
    let (uid, gsiv) = (u64_at(h, 16), u32::from_le_bytes([h[24], h[25], h[26], h[27]]));
    Some(Platform { class, irq_flags, flags, hid, uid, gsiv, windows, path: path.to_vec() })
}
