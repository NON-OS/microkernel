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

//! One lie told in a sound disk. A field lie is resealed, so its CRCs are
//! right over the wrong contents and only the field can refuse it; a CRC
//! lie is told with nothing resealed.

use super::disk::Disk;
use super::entry_lie::entry_lie;
use super::head_lie::head_lie;
pub use super::lies::{Lie, LIES};
use super::rng::Rng;

pub fn tell(r: &mut Rng, d: &Disk) -> (Lie, Disk) {
    loop {
        let (lie, mut told) = (LIES[r.range(0, LIES.len() as u64 - 1) as usize], d.clone());
        if apply(r, lie, &mut told).is_some() {
            return (lie, told);
        }
    }
}

fn apply(r: &mut Rng, lie: Lie, d: &mut Disk) -> Option<()> {
    let (n, size) = (d.get(80, 4), d.get(84, 4));
    match lie {
        Lie::HeaderCrc => {
            let at = [r.range(0, 15), r.range(20, 91)][r.range(0, 1) as usize];
            d.header[at as usize] ^= 1 << r.range(0, 7);
            return Some(());
        }
        Lie::ArrayCrc => {
            d.array[r.range(0, n * size - 1) as usize] ^= 1 << r.range(0, 7);
            return Some(());
        }
        Lie::NoMbrSignature => d.mbr[510 + r.range(0, 1) as usize] ^= r.range(1, 255) as u8,
        Lie::NoProtective => d.mbr[446..510]
            .chunks_mut(16)
            .for_each(|e| e[4] = if e[4] == 0xEE { 0x07 } else { e[4] }),
        Lie::ProtectiveNotAtOne => {
            d.mbr[446..510].chunks_mut(16).filter(|e| e[4] == 0xEE).for_each(|e| e[8] ^= 3)
        }
        Lie::Block4096 => d.header.fill(0),
        Lie::Overlap | Lie::PastLast | Lie::EndBeforeStart | Lie::BelowFirst => {
            entry_lie(r, lie, d)?
        }
        _ => head_lie(r, lie, d),
    }
    d.seal();
    Some(())
}
