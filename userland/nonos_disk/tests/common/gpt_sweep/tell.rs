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

//! Telling one lie in a sound disk, in either header.

use super::disk::Disk;
use super::lie::{apply, LIES};
use super::lies::Lie;
use super::rng::Rng;

/// The lie, the header it was told in, whether the primary was broken too,
/// and the disk as told. A lie in the backup is seen only when the primary
/// fails, so one time in two the primary's signature is broken as well.
pub fn tell(r: &mut Rng, d: &Disk) -> (Lie, usize, bool, Disk) {
    let (lie, w, mut told) = tell_one(r, d);
    let broke = w == 1 && r.one_in(2);
    if broke {
        told.heads[0][60] ^= 1; // its CRC fails; its signature stays
    }
    (lie, w, broke, told)
}

fn tell_one(r: &mut Rng, d: &Disk) -> (Lie, usize, Disk) {
    loop {
        let (lie, w) = (LIES[r.range(0, LIES.len() as u64 - 1) as usize], r.range(0, 1) as usize);
        let mut told = d.clone();
        if let Some(w) = apply(r, lie, w, &mut told) {
            return (lie, w, told);
        }
    }
}
