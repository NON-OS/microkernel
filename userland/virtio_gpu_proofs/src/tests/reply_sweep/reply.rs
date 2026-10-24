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

//! A well-formed reply for each command, its words at edges or noise;
//! `mutate` then breaks it.

use crate::tests::part::answers::header;
use crate::tests::wire_sweep::rng::Rng;

pub fn display(r: &mut Rng) -> Vec<u8> {
    let mut out = header(0x1101, 0, 0);
    for _ in 0..16 {
        let on = r.below(3) != 0;
        for v in [r.word(), r.word(), r.word(), r.word(), on as u32, 0] {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn edid(r: &mut Rng) -> Vec<u8> {
    let size = [128u32, 256, 384, 1024][r.below(4) as usize];
    let mut out = header(0x1104, 0, 0);
    out.extend_from_slice(&size.to_le_bytes());
    out.extend_from_slice(&[0; 4]);
    let mut blob: Vec<u8> = (0..size).map(|_| r.next() as u8).collect();
    blob[..8].copy_from_slice(&[0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0]);
    out.extend_from_slice(&blob);
    out
}

pub fn capset(r: &mut Rng) -> Vec<u8> {
    let mut out = header(0x1102, 0, 0);
    [r.word(), r.word(), r.word(), 0].iter().for_each(|v| out.extend_from_slice(&v.to_le_bytes()));
    out
}
