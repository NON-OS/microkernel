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

//! One lie told in a sound stream at frame `j`, and the frames that must
//! still be handed up whole: all but frame `j` when a payload of it lies,
//! all but `j` and the next when a header with no FID to read hid `j`'s EOF,
//! all of them for the stream's honest liberties.

use super::build::{stream, Stream};
use super::frame_lie::frame_lie;
pub use super::lies::{Lie, LIES};
use super::rng::Rng;

pub struct Told {
    pub lie: Lie,
    pub s: Stream,
    pub expect: Vec<Vec<u8>>,
}

pub fn tell(r: &mut Rng) -> Told {
    let lie = LIES[r.range(0, LIES.len() - 1)];
    let mut s = stream(r);
    let n = s.frames.len();
    let j = r.range(0, n - 1);
    let of_j: Vec<usize> = (0..s.payloads.len()).filter(|&i| s.payloads[i].frame == j).collect();
    let (first, last) = (of_j[0], of_j[of_j.len() - 1]);
    let pick = of_j[r.range(0, of_j.len() - 1)];
    let Some(dropped) = frame_lie(r, lie, &mut s, (j, first, last, pick)) else {
        return tell(r);
    };
    let expect = (0..n).filter(|f| !dropped.contains(f)).map(|f| s.frames[f].clone()).collect();
    Told { lie, s, expect }
}
