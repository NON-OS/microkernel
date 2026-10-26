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

//! Every scenario: each directory shape, each pair of operations, H cut at
//! each of its requests or not at all, its request in flight landing or
//! not. The marks of every scenario, joined, and how many there were.

use super::disk::{Disk, FIRST_FREE};
use super::kinds::{Dir, Fate, Knob, Mount, Op};
use super::run::{run, scenario};

/// How many requests H sends running `h` alone to its end.
fn requests(dir: Dir, h: Op) -> usize {
    let (mut disk, mut mount) = (Disk::new(dir), Mount { generation: 1, free: FIRST_FREE });
    let mut r = run(&mut disk, &mut mount, dir);
    r.op(h);
    r.sent
}

pub(super) fn search(k: Knob) -> (u8, usize) {
    let (mut bad, mut n) = (0, 0);
    for dir in [Dir::HasRoom, Dir::Full, Dir::NoRecord] {
        for h in [Op::Create, Op::Unlink, Op::Close] {
            for c in [Op::Create, Op::Unlink, Op::Close] {
                let stops = (0..requests(dir, h)).map(Some).chain([None]);
                for stop in stops {
                    for fate in [Fate::Lands, Fate::Never] {
                        bad |= scenario(dir, h, c, stop, fate, k);
                        n += 1;
                    }
                }
            }
        }
    }
    (bad, n)
}
