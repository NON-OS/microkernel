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

//! A lie told in frame `j` of a stream, and the frames it spoils.

use super::build::{payload, Payload, Stream, EOF, ERR};
use super::liberty::liberty;
use super::lies::Lie;
use super::rng::Rng;

/// `j`, its first and last payload, and a payload of it picked at random.
pub fn frame_lie(
    r: &mut Rng,
    lie: Lie,
    s: &mut Stream,
    at: (usize, usize, usize, usize),
) -> Option<Vec<usize>> {
    let (j, first, last, pick) = at;
    let dropped = match lie {
        Lie::ErrBit => (s.payloads[pick].bytes[1] |= ERR, vec![j]).1,
        Lie::HeaderShort | Lie::HeaderPast => {
            let p = &mut s.payloads[pick].bytes;
            p[0] = if lie == Lie::HeaderShort {
                r.range(0, 1) as u8
            } else {
                (p.len() + r.range(1, 9)).min(255) as u8
            };
            if p[0] as usize <= p.len() && lie == Lie::HeaderPast {
                return None;
            }
            // No FID to read: its frame is spoiled, and when it carried that
            // frame's EOF the next frame begins spoiled too.
            if pick == last {
                vec![j, j + 1]
            } else {
                vec![j]
            }
        }
        Lie::CutShort => (s.payloads[last].bytes[1] &= !EOF, vec![j]).1,
        Lie::Oversize => {
            let extra = payload(r, s.payloads[last].bytes[1] & 1, &vec![9; s.max]);
            s.payloads.insert(last, Payload { bytes: extra, frame: j });
            vec![j]
        }
        _ => return liberty(r, lie, s, (j, first, last)),
    };
    Some(dropped)
}
