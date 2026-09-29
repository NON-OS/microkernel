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

use super::bits::Bools;
use super::quant_tables::{AC_Q, DC_Q};

/// Per-segment values and whether they replace (true) or add to the frame's.
pub(super) type Segmented = Option<(bool, [i32; 4])>;

/// Loop-filter settings for one kind of macroblock (RFC 6386 15.2).
#[derive(Clone, Copy, Default)]
pub(super) struct Strength {
    pub limit: i32,
    pub ilevel: i32,
    pub hev: i32,
    pub inner: bool,
}

/// The segment header (RFC 6386 9.3): the map's tree probabilities when
/// it is sent, and the per-segment quantizer and filter levels.
pub(super) fn segments(br: &mut Bools) -> (Option<[u8; 3]>, Segmented, Segmented) {
    if !br.bit(128) {
        return (None, None, None);
    }
    let map = br.bit(128);
    let (mut absolute, mut q, mut f) = (true, [0; 4], [0; 4]);
    if br.bit(128) {
        absolute = br.bit(128);
        q = [0; 4].map(|_| br.opt_signed(7));
        f = [0; 4].map(|_| br.opt_signed(6));
    }
    let probs = map.then(|| [0; 3].map(|_| if br.bit(128) { br.literal(8) as u8 } else { 255 }));
    (probs, Some((absolute, q)), Some((absolute, f)))
}

/// Base quantizer index and deltas (RFC 6386 9.6) to step sizes per segment.
pub(super) fn quantizers(br: &mut Bools, seg: Segmented) -> [[i32; 6]; 4] {
    let base = br.literal(7) as i32;
    let d = [0; 5].map(|_| br.opt_signed(4));
    let dc = |i: i32, max: i32| DC_Q[i.clamp(0, max) as usize] as i32;
    let ac = |i: i32| AC_Q[i.clamp(0, 127) as usize] as i32;
    [0, 1, 2, 3].map(|s| {
        let q = match seg {
            Some((true, v)) => v[s],
            Some((false, v)) => v[s] + base,
            None => base,
        };
        [
            dc(q + d[0], 127),
            ac(q),
            dc(q + d[1], 127) * 2,
            (ac(q + d[2]) * 101_581 >> 16).max(8),
            dc(q + d[3], 117),
            ac(q + d[4]),
        ]
    })
}
