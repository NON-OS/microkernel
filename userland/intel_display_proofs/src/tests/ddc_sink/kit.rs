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

//! What the edge proofs share: the route, the read's deadline, an honest
//! EDID of so many blocks, and one read with what it left.

use super::own::{BLOCK, HEADER};
use super::sink::Sink;
use crate::display::{read_edid, EDID_MAX};
use crate::engine::route::{Link, Route};
use crate::model::Ticks;

/// The driver's whole-read deadline, as its letter states it.
pub const EDID_US: u64 = 400_000;
pub const HDMI_B: Route = Route { ddi: 1, link: Link::Hdmi };

/// `n` distinct blocks, the base declaring the rest, each sealed.
pub fn honest(n: usize) -> Vec<u8> {
    let mut e: Vec<u8> = (0..n * BLOCK).map(|i| (i * 7 + i / BLOCK) as u8).collect();
    e[..8].copy_from_slice(&HEADER);
    e[126] = (n - 1) as u8;
    for b in e.chunks_mut(BLOCK) {
        b[127] = 0;
        b[127] = b.iter().fold(0u8, |a, v| a.wrapping_add(*v)).wrapping_neg();
    }
    e
}

/// One read: the bytes kept, the whole buffer, and the time it took.
pub fn read(s: &mut Sink, t: &Ticks) -> (usize, Vec<u8>, u64) {
    let mut out = [0xA5u8; EDID_MAX];
    let start = t.now.get();
    let n = read_edid(s, t, &HDMI_B, &mut out);
    (n, out.to_vec(), t.now.get() - start)
}
