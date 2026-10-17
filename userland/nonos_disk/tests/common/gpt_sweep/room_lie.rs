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

//! An array that breaks only its room or only its size, every other field
//! sound: one sector past where it may end, or one entry past 128 KiB with
//! room made for it. A moved array is moved whole.

use super::disk::Disk;
use super::lies::Lie;
use super::rng::Rng;

pub fn room_lie(r: &mut Rng, lie: Lie, w: usize, d: &mut Disk) {
    let (first, last, lba) = (d.get(w, 40, 8), d.get(w, 48, 8), d.get(w, 72, 8));
    let size = d.get(w, 84, 4);
    let s = (d.get(w, 80, 4) * size).div_ceil(512);
    let moved = |d: &mut Disk, at: u64| {
        d.put(w, 72, at, 8);
        d.array_lba[w] = at;
    };
    match (lie, w) {
        // FirstUsableLBA on the array's last sector.
        (Lie::ArrayPastRoom, 0) => d.put(0, 40, lba + s - 1, 8),
        // The backup's array on its header, or on LastUsableLBA.
        (Lie::ArrayPastRoom, _) => moved(d, r.one_of(&[d.sectors - s, last])),
        (Lie::ArrayTooBig, _) => {
            let n = 128 * 1024 / size + 1;
            let s = (n * size).div_ceil(512);
            d.put(w, 80, n, 4);
            match w {
                0 => d.put(0, 40, first.max(lba + s), 8),
                _ => {
                    let at = d.sectors - 1 - s;
                    moved(d, at);
                    d.put(1, 48, last.min(at - 1), 8);
                }
            }
        }
        other => unreachable!("{other:?} is not a room lie"),
    }
}
