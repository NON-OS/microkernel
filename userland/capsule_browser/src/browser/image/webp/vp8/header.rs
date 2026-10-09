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

use alloc::vec::Vec;

use super::bits::Bools;
use super::fields::{quantizers, segments, Strength};
use super::filter_hdr::filter_strengths;
use super::partitions::partitions;
use super::token_probs::token_probs;

/// A key frame's header, read from the frame tag and partition 0.
pub(super) struct Header<'a> {
    pub w: usize,
    pub h: usize,
    pub mbw: usize,
    pub mbh: usize,
    /* Segment-id tree probabilities, when the frame carries a segment map. */
    pub seg_map: Option<[u8; 3]>,
    /* Per segment: [y1 dc, y1 ac, y2 dc, y2 ac, uv dc, uv ac] steps. */
    pub quant: [[i32; 6]; 4],
    /* 0 no loop filter, 1 simple, 2 normal; strengths by segment and
     * whether the macroblock is predicted 4x4. */
    pub filter: u8,
    pub strength: [[Strength; 2]; 4],
    pub probs: Vec<u8>,
    pub skip_prob: Option<u8>,
    pub parts: Vec<&'a [u8]>,
}

/// Parse a VP8 key frame's headers. Returns them with the partition-0
/// decoder positioned at the first macroblock's modes.
pub(super) fn parse(data: &[u8]) -> Option<(Header<'_>, Bools<'_>)> {
    let b = data.get(..10)?;
    let tag = b[0] as u32 | (b[1] as u32) << 8 | (b[2] as u32) << 16;
    if tag & 1 != 0 || (tag >> 1) & 7 > 3 || (tag >> 4) & 1 == 0 || b[3..6] != [0x9D, 0x01, 0x2A] {
        return None;
    }
    let w = (b[6] as usize | (b[7] as usize) << 8) & 0x3FFF;
    let h = (b[8] as usize | (b[9] as usize) << 8) & 0x3FFF;
    let first = (tag >> 5) as usize;
    let rest = data.get(10..)?;
    if w == 0 || h == 0 || first > rest.len() {
        return None;
    }
    let mut br = Bools::new(&rest[..first]);
    br.literal(2);
    let (seg_map, seg_q, seg_f) = segments(&mut br);
    let (filter, strength) = filter_strengths(&mut br, seg_f);
    let parts = partitions(&mut br, &rest[first..])?;
    let quant = quantizers(&mut br, seg_q);
    br.literal(1);
    let probs = token_probs(&mut br);
    let skip_prob = br.bit(128).then(|| br.literal(8) as u8);
    let (mbw, mbh) = (w.div_ceil(16), h.div_ceil(16));
    let hdr = Header { w, h, mbw, mbh, seg_map, quant, filter, strength, probs, skip_prob, parts };
    (!br.eof).then_some((hdr, br))
}
