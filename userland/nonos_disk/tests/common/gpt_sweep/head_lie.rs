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

//! A header field told wrong, each to a value the rule refuses.

use super::disk::Disk;
use super::lies::Lie;
use super::rng::Rng;

pub fn head_lie(r: &mut Rng, lie: Lie, d: &mut Disk) {
    let (first, last, lba) = (d.get(40, 8), d.get(48, 8), d.get(72, 8));
    let (n, size, sectors) = (d.get(80, 4), d.get(84, 4), d.sectors);
    match lie {
        Lie::Signature => d.header[r.range(0, 7) as usize] ^= r.range(1, 255) as u8,
        Lie::HeaderSize => {
            let v = [r.range(0, 91), r.range(513, u32::MAX as u64)];
            d.put(12, r.one_of(&v), 4)
        }
        Lie::MyLba => {
            let v = [0, 2, sectors - 1, r.next() | 2];
            d.put(24, r.one_of(&v), 8)
        }
        Lie::FirstAfterLast => d.put(40, last + r.range(1, 1000), 8),
        Lie::LastPastDisk => d.put(48, sectors + r.range(0, 1 << 20), 8),
        Lie::EntriesLba => {
            let v = [0, 1, first, u64::MAX - r.range(0, 64)];
            d.put(72, r.one_of(&v), 8)
        }
        Lie::EntryCount => {
            let past = (first - lba) * 512 / size + r.range(1, 64);
            let v = [0, past, r.range(1 << 20, u32::MAX as u64)];
            d.put(80, r.one_of(&v), 4)
        }
        Lie::EntrySize => {
            let big = (256 * 1024 / n).next_power_of_two().max(256);
            let v = [r.range(0, 127), r.range(129, 4095) | 1, big];
            d.put(84, r.one_of(&v), 4)
        }
        other => unreachable!("{other:?} is not a header field"),
    }
}
