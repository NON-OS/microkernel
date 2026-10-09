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

//! A repeated match: one of the last four distances again, or a single byte
//! from the last one.

use super::lzma_model::Model;
use super::range::Range;

const G0: usize = 1;
const G1: usize = 2;
const G2: usize = 3;

pub enum Rep {
    /// One byte from rep0, no length read.
    Short,
    /// A length, less two, from rep0 as it now stands.
    Len(u32),
}

pub fn rep(rc: &mut Range, m: &mut Model, s: usize, pos: usize) -> Rep {
    if rc.bit(&mut m.is_rep[G0][s]) == 0 {
        if rc.bit(&mut m.is_rep0_long[(s << 4) + pos]) == 0 {
            m.state = if s < 7 { 9 } else { 11 };
            return Rep::Short;
        }
    } else {
        let dist = if rc.bit(&mut m.is_rep[G1][s]) == 0 {
            m.rep[1]
        } else {
            let d = match rc.bit(&mut m.is_rep[G2][s]) {
                0 => m.rep[2],
                _ => {
                    let d = m.rep[3];
                    m.rep[3] = m.rep[2];
                    d
                }
            };
            m.rep[2] = m.rep[1];
            d
        };
        m.rep[1] = m.rep[0];
        m.rep[0] = dist;
    }
    m.state = if s < 7 { 8 } else { 11 };
    Rep::Len(m.rep_len.decode(rc, pos))
}
