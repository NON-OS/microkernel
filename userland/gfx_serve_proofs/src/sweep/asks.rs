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

//! The frame the sweep sends, sometimes with a lie in the frame itself: a
//! length past its payload, a magic or version wrong, the frame cut short
//! or run on.

use super::kit::{Header, HDR_LEN};
use super::rng::Rng;

pub fn frame(r: &mut Rng, h: Header, p: &[u8]) -> Vec<u8> {
    let said = if r.below(8) == 0 { r.next() as u32 % 300 } else { p.len() as u32 };
    let mut f = vec![0u8; HDR_LEN];
    Header { payload_len: said, ..h }.encode(&mut f).unwrap();
    f.extend_from_slice(p);
    match r.below(16) {
        0 => f[r.below(6) as usize] ^= 1,
        1 => f.truncate(r.below(f.len() as u32) as usize),
        2 => f.extend((0..r.below(300)).map(|_| r.next() as u8)),
        _ => {}
    }
    f
}
