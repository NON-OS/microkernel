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

/* data/streams.dat holds streams made by the reference encoder (python
brotli 1.2.0), each after a header: input kind, input size, seed,
quality, window bits, mode, stream size. */

use super::inputs::input;

const STREAMS: &[u8] = include_bytes!("../data/streams.dat");

pub struct Case {
    pub quality: u8,
    pub lgwin: u8,
    pub mode: u8,
    pub stream: &'static [u8],
    pub plain: Vec<u8>,
}

impl Case {
    pub fn name(&self) -> String {
        let (q, w, m, n) = (self.quality, self.lgwin, self.mode, self.plain.len());
        format!("q{q} lgwin{w} mode{m} {n} bytes")
    }
}

pub fn cases() -> Vec<Case> {
    let (mut all, mut at) = (Vec::new(), 0);
    while at < STREAMS.len() {
        let h = &STREAMS[at..at + 13];
        let word = |i: usize| u32::from_le_bytes([h[i], h[i + 1], h[i + 2], h[i + 3]]) as usize;
        let (n, len) = (word(1), word(9));
        let stream = &STREAMS[at + 13..at + 13 + len];
        let plain = input(h[0], n, h[5] as u64);
        all.push(Case { quality: h[6], lgwin: h[7], mode: h[8], stream, plain });
        at += 13 + len;
    }
    all
}
