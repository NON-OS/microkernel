// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The hostile codec's answers: for a read the codec's description does not
//! define, a word that depends only on the seed and the question, so a run
//! is repeatable and every question gets the same answer each time it is
//! asked, as a real codec's registers would.

use super::SimCodec;

/// The word for (`nid`, `verb`, `payload`) when `codec` is noisy and does not
/// define that read itself; None when it does, or is not noisy.
pub(super) fn undefined(codec: &SimCodec, nid: u8, verb: u32, payload: u16) -> Option<u32> {
    let seed = codec.noise?;
    let node = codec.nodes.get(&nid);
    let defined = match verb {
        0xf00 => node.is_some_and(|n| n.params.contains_key(&payload)),
        0xf02 => node.is_some_and(|n| !n.conn.is_empty()),
        0xf1c => node.is_some_and(|n| n.pin_cfg != 0),
        0xf20 => codec.subsystem != 0,
        _ => true,
    };
    (!defined).then(|| mix(seed ^ (nid as u64) << 40 ^ (verb as u64) << 16 ^ payload as u64) as u32)
}

/// splitmix64's finaliser: every input bit reaches every output bit.
pub fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
