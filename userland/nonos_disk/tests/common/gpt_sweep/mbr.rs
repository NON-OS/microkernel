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

//! Sector 0 of a sound GPT disk.

use super::rng::Rng;

/// A protective MBR: one EEh record from LBA 1, in any of the four slots,
/// the others empty or some other system's, as a hybrid MBR carries them.
pub fn mbr(r: &mut Rng, sectors: u64) -> Vec<u8> {
    let mut s = r.bytes(512);
    s[446..510].fill(0);
    for slot in 0..4 {
        s[446 + 16 * slot + 4] = if r.one_in(4) { r.range(1, 0xED) as u8 } else { 0 };
    }
    let e = &mut s[446 + 16 * r.range(0, 3) as usize..][..16];
    e[4] = 0xEE;
    e[8..12].copy_from_slice(&1u32.to_le_bytes());
    e[12..16].copy_from_slice(&((sectors - 1).min(u32::MAX as u64) as u32).to_le_bytes());
    (s[510], s[511]) = (0x55, 0xAA);
    s
}
