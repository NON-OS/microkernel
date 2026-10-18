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

//! One lie told in a sound disk, in the primary header (0) or the backup
//! (1). A field lie is resealed, so its CRCs are right over the wrong
//! contents and only the field can refuse it; a CRC lie is told with
//! nothing resealed.

use super::backup_lie::backup_lie;
use super::disk::Disk;
use super::entry_lie::entry_lie;
use super::head_lie::{head_lie, FIELDS};
pub use super::lies::{Lie, LIES};
use super::mbr::mbr_lie;
use super::rng::Rng;
use super::room_lie::room_lie;
pub use super::tell::tell;

pub fn apply(r: &mut Rng, lie: Lie, w: usize, d: &mut Disk) -> Option<usize> {
    let bytes = d.get(w, 80, 4) * d.get(w, 84, 4);
    let flip = 1u8 << r.range(0, 7);
    match lie {
        Lie::HeaderCrc => {
            d.heads[w][[r.range(0, 15), r.range(20, 91)][r.range(0, 1) as usize] as usize] ^= flip
        }
        Lie::ArrayCrc => d.arrays[w][r.range(0, bytes - 1) as usize] ^= flip,
        Lie::NoMbrSignature | Lie::NoProtective | Lie::ProtectiveNotAtOne => mbr_lie(r, lie, d),
        Lie::Block4096 => d.heads.iter_mut().for_each(|h| h.fill(0)),
        Lie::BothBroken => {
            let field = FIELDS[r.range(0, FIELDS.len() as u64 - 1) as usize];
            for w in 0..2 {
                head_lie(r, field, w, d);
                d.seal(w);
            }
        }
        Lie::ArrayPastRoom | Lie::ArrayTooBig => {
            room_lie(r, lie, w, d);
            d.seal(w);
        }
        Lie::Overlap | Lie::PastLast | Lie::EndBeforeStart | Lie::BelowFirst => {
            entry_lie(r, lie, w, d)?;
            d.seal(w);
        }
        Lie::BackupMyLba | Lie::BackupAlternate | Lie::BackupArrayOverHeader => {
            return backup_lie(r, lie, d)
        }
        Lie::BackupArrayInUsable | Lie::Disagree => return backup_lie(r, lie, d),
        _ => {
            head_lie(r, lie, w, d);
            d.seal(w);
        }
    }
    Some(w)
}
