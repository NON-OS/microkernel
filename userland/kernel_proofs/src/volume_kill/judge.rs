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

//! The judge, after the reboot: walk what D and N reach, and ask whether
//! C's operation is still there.

use std::collections::BTreeSet;

use super::disk::{Disk, D, N};
use super::kinds::Op;
use super::ops::{CHILD, INDEX};
use super::run::{ABOVE, LOST, UNWRITTEN};

pub(super) fn judge(disk: &Disk, c: Op, slot: [u64; 4]) -> u8 {
    let free = disk.reboot_free();
    let (mut seen, mut todo, mut bad) = (BTreeSet::new(), vec![D, N], 0);
    while let Some(b) = todo.pop() {
        if !seen.insert(b) {
            continue;
        }
        for t in disk.points(b) {
            bad |= ABOVE * u8::from(t >= free);
            bad |= UNWRITTEN * u8::from(!disk.blocks.contains_key(&t));
            todo.push(t);
        }
    }
    let named = |r: &u64| disk.recs.contains(r) && disk.points(*r).contains(&N);
    let kept = match c {
        Op::Create => seen.contains(&slot[CHILD as usize]),
        Op::Unlink => !seen.iter().any(named),
        Op::Close => disk.points(N).contains(&slot[INDEX as usize]),
    };
    bad | (LOST * u8::from(!kept))
}
