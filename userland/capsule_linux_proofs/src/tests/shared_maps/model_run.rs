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

//! Random calls from three processes on two files: after each, every map
//! shows its file as a Linux page cache would, zero past the end; every read
//! and every flush sees what the maps wrote; and the one writer rule and
//! the bound hold.

use super::super::random::Regs;
use super::model::{World, PATHS};
use super::support::P;
use crate::linux::file::shared_maps::asks::{protects, writers};
use crate::linux::file::shared_maps::names::unlink;

fn step(w: &mut World, r: &mut Regs) {
    let (pid, i, at) = (r.small(3) as u32, r.small(2) as usize, r.small(12) * P);
    match r.small(10) {
        0..=2 => super::model_guest::map_one(w, r, pid, i),
        3 => w.write(i, r.small(8 * P), &vec![1 + r.small(255) as u8; r.small(2 * P) as usize]),
        4 => {
            let t = w.truth[i].clone();
            let (lo, len) = (r.small(10 * P), r.small(3 * P));
            let want = &t[(lo as usize).min(t.len())..((lo + len) as usize).min(t.len())];
            assert_eq!(w.read(i, lo, len), want, "a read sees what the maps wrote");
        }
        5 => w.truncate(i, r.small(8 * P)),
        6 => drop(w.unmap(pid, at, r.small(4) * P)),
        7 => {
            w.flush(i);
            assert_eq!(w.copy[i], w.truth[i], "a flush puts in what the maps wrote");
        }
        8 if r.small(8) == 0 => {
            unlink(&mut w.table, PATHS[i]);
            (w.truth[i], w.copy[i]) = (vec![], vec![]);
        }
        8 if protects(&w.table, pid, at, at + 4 * P, (true, false)).is_ok() => {
            writers(&mut w.table, pid, at, at + 4 * P)
        }
        _ => super::model_guest::store(w, r),
    }
}

#[test]
fn every_map_shows_its_file_and_every_read_sees_what_the_maps_wrote() {
    let mut r = Regs::new(0x5A12_ED00);
    for _ in 0..150 {
        let mut w = World::new(3, 32);
        for _ in 0..150 {
            step(&mut w, &mut r);
            super::model_check::check(&w);
        }
    }
}
