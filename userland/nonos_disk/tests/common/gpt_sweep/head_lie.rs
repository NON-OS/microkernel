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

//! A header field told wrong, in the primary (0) or the backup (1), each
//! to a value the rule refuses for that header.

use super::disk::Disk;
use super::lies::Lie;
use super::rng::Rng;

/// The field lies, which "both broken" tells in each header.
pub const FIELDS: [Lie; 8] = [
    Lie::Signature,
    Lie::HeaderSize,
    Lie::MyLba,
    Lie::FirstAfterLast,
    Lie::LastPastDisk,
    Lie::EntriesLba,
    Lie::EntryCount,
    Lie::EntrySize,
];

pub fn head_lie(r: &mut Rng, lie: Lie, w: usize, d: &mut Disk) {
    let (first, last, lba) = (d.get(w, 40, 8), d.get(w, 48, 8), d.get(w, 72, 8));
    let (n, size, sectors) = (d.get(w, 80, 4), d.get(w, 84, 4), d.sectors);
    let room_end = [first, sectors - 1][w]; // FirstUsableLBA, or the backup
    let v = match lie {
        Lie::Signature => return d.heads[w][r.range(0, 7) as usize] ^= r.range(1, 255) as u8,
        Lie::HeaderSize => [r.range(0, 91), r.range(513, u32::MAX as u64)].to_vec(),
        Lie::MyLba => {
            [vec![0, 2, sectors - 1, r.next() | 2], vec![0, 1, sectors - 2, sectors]][w].clone()
        }
        Lie::FirstAfterLast => vec![last + r.range(1, 1000)],
        Lie::LastPastDisk => vec![sectors + r.range(0, 1 << 20)],
        Lie::EntriesLba => vec![0, 1, [first, last][w], u64::MAX - r.range(0, 64)],
        Lie::EntryCount => vec![
            0,
            (room_end - lba) * 512 / size + r.range(1, 64),
            r.range(1 << 20, u32::MAX as u64),
        ],
        Lie::EntrySize => {
            let big = (256 * 1024 / n).next_power_of_two().max(256);
            vec![r.range(0, 127), r.range(129, 4095) | 1, big]
        }
        other => unreachable!("{other:?} is not a header field"),
    };
    let (at, width) = match lie {
        Lie::HeaderSize => (12, 4),
        Lie::MyLba => (24, 8),
        Lie::FirstAfterLast => (40, 8),
        Lie::LastPastDisk => (48, 8),
        Lie::EntriesLba => (72, 8),
        Lie::EntryCount => (80, 4),
        _ => (84, 4),
    };
    let told = r.one_of(&v);
    d.put(w, at, told, width);
    // An array named elsewhere is moved there whole: only its place refuses.
    if lie == Lie::EntriesLba {
        d.array_lba[w] = told;
    }
}
