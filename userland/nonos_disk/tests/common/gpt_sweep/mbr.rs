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

use super::lies::Lie;
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

/// Sector 0 told wrong: no 55h AAh, no EEh record, or one not at LBA 1.
pub fn mbr_lie(r: &mut Rng, lie: Lie, d: &mut super::disk::Disk) {
    let records = d.mbr[446..510].chunks_mut(16);
    match lie {
        Lie::NoMbrSignature => d.mbr[510 + r.range(0, 1) as usize] ^= r.range(1, 255) as u8,
        Lie::NoProtective => d.mbr[446..510]
            .chunks_mut(16)
            .for_each(|e| e[4] = if e[4] == 0xEE { 0x07 } else { e[4] }),
        Lie::ProtectiveNotAtOne => records.filter(|e| e[4] == 0xEE).for_each(|e| e[8] ^= 3),
        other => unreachable!("{other:?} is not an MBR lie"),
    }
}
