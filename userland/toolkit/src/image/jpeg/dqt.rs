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

use crate::image::types::DecodeError;

pub const MAX_QT: usize = 4;

#[derive(Clone, Copy)]
pub struct QuantTable {
    pub present: bool,
    pub values: [u16; 64],
}

impl QuantTable {
    pub const fn new() -> Self {
        Self { present: false, values: [0; 64] }
    }
}

pub fn parse_dqt(seg: &[u8], tables: &mut [QuantTable; MAX_QT]) -> Result<(), DecodeError> {
    let mut p = 0usize;
    while p < seg.len() {
        let pq_tq = seg[p];
        p += 1;
        let pq = (pq_tq >> 4) & 0x0F;
        let tq = (pq_tq & 0x0F) as usize;
        if tq >= MAX_QT {
            return Err(DecodeError::Unsupported);
        }
        let entry_bytes = if pq == 0 {
            64
        } else if pq == 1 {
            128
        } else {
            return Err(DecodeError::Unsupported);
        };
        if p + entry_bytes > seg.len() {
            return Err(DecodeError::Truncated);
        }
        let mut t = QuantTable::new();
        /*
         * Kept in the file's zigzag order, the order both decoders index it
         * in: the streaming one by scan position, the coefficient one when it
         * builds its natural-order copy.
         */
        for (i, v) in t.values.iter_mut().enumerate() {
            *v = if pq == 0 {
                seg[p + i] as u16
            } else {
                ((seg[p + i * 2] as u16) << 8) | seg[p + i * 2 + 1] as u16
            };
        }
        t.present = true;
        tables[tq] = t;
        p += entry_bytes;
    }
    Ok(())
}
