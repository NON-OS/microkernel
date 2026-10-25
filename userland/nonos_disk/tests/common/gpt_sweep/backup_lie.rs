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

//! Lies only the backup can tell, each resealed: its own LBA or the
//! primary's named wrong, its array over the backup header or inside the
//! usable range, and a backup that holds but disagrees with the primary.

use super::disk::Disk;
use super::lies::Lie;
use super::rng::Rng;

pub fn backup_lie(r: &mut Rng, lie: Lie, d: &mut Disk) -> Option<usize> {
    let (first, last, sectors) = (d.get(1, 40, 8), d.get(1, 48, 8), d.sectors);
    let s = (d.get(1, 80, 4) * d.get(1, 84, 4)).div_ceil(512);
    match lie {
        Lie::BackupMyLba => {
            let v = [0, 1, sectors - 2, sectors, r.next() | (1 << 63)];
            d.put(1, 24, r.one_of(&v), 8);
        }
        Lie::BackupAlternate => {
            let v = [0, 2, sectors - 1, r.next() | 2];
            d.put(1, 32, r.one_of(&v), 8);
        }
        Lie::BackupArrayOverHeader => d.array_lba[1] = r.range(sectors - s, sectors - 1),
        Lie::BackupArrayInUsable => d.array_lba[1] = r.range(first, last),
        Lie::Disagree => match r.range(0, 2) {
            0 => d.heads[1][56 + r.range(0, 15) as usize] ^= 1 << r.range(0, 7),
            1 => d.put(1, 48, last - 1, 8),
            _ => d.entry(1, 0)[56 + r.range(0, 71) as usize] ^= 1 << r.range(0, 7),
        },
        other => unreachable!("{other:?} is not the backup's own lie"),
    }
    if matches!(lie, Lie::BackupArrayOverHeader | Lie::BackupArrayInUsable) {
        d.put(1, 72, d.array_lba[1], 8);
    }
    d.seal(1);
    Some(1)
}
