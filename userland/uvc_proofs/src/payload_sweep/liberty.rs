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

//! The stream's honest liberties, after which every frame still comes whole:
//! empty payloads, an EOF on a header-only payload, a FID left unflipped
//! across an EOF, a frame of no bytes.

use super::build::{payload, Payload, Stream, EOF};
use super::lies::Lie;
use super::rng::Rng;

pub fn liberty(
    r: &mut Rng,
    lie: Lie,
    s: &mut Stream,
    at: (usize, usize, usize),
) -> Option<Vec<usize>> {
    let (j, first, last) = at;
    let n = s.frames.len();
    match lie {
        Lie::EmptyPayloads => {
            for _ in 0..r.range(1, 4) {
                let at = r.range(0, s.payloads.len());
                s.payloads.insert(at, Payload { bytes: Vec::new(), frame: usize::MAX });
            }
        }
        Lie::HeaderOnlyEof => {
            let info = s.payloads[last].bytes[1];
            s.payloads[last].bytes[1] &= !EOF;
            s.payloads
                .insert(last + 1, Payload { bytes: payload(r, info & 1 | EOF, &[]), frame: j });
        }
        Lie::NoFlip if j + 1 < n => {
            let fid = s.payloads[first].bytes[1] & 1;
            s.payloads
                .iter_mut()
                .filter(|p| p.frame == j + 1)
                .for_each(|p| p.bytes[1] = p.bytes[1] & !1 | fid);
        }
        Lie::NoFlip => return None,
        Lie::EmptyFrame => {
            // After frame j ends, a frame of no bytes: nothing to hand up.
            let info = s.payloads[last].bytes[1] & 1 | EOF;
            s.payloads.insert(last + 1, Payload { bytes: payload(r, info, &[]), frame: j });
        }
        _ => unreachable!("{lie:?} is not a liberty"),
    };
    Some(Vec::new())
}
