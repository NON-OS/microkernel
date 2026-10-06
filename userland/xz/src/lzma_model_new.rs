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

//! Making a model from its properties byte, and resetting one.

use alloc::vec;
use alloc::vec::Vec;

use super::lzma_len::Len;
use super::lzma_model::{Model, HALF};

impl Model {
    /// From the properties byte, `(pb * 5 + lp) * 9 + lc`; LZMA2 holds
    /// lc + lp to four.
    pub fn new(props: u8) -> Option<Model> {
        let (lc, lp, pb) = (u32::from(props % 9), u32::from(props / 9 % 5), u32::from(props / 45));
        if pb > 4 || lc + lp > 4 {
            return None;
        }
        Some(Model::fresh(lc, lp, pb, vec![HALF; 0x300 << (lc + lp)]))
    }

    /// A state reset: every probability back to even, no history.
    pub fn reset(&mut self) {
        let mut literal = core::mem::take(&mut self.literal);
        literal.fill(HALF);
        *self = Model::fresh(self.lc, self.lp, self.pb, literal);
    }

    fn fresh(lc: u32, lp: u32, pb: u32, literal: Vec<u16>) -> Model {
        Model {
            lc,
            lp,
            pb,
            state: 0,
            rep: [0; 4],
            is_match: [HALF; 192],
            is_rep: [[HALF; 12]; 4],
            is_rep0_long: [HALF; 192],
            pos_slot: [[HALF; 64]; 4],
            pos: [HALF; 115],
            align: [HALF; 16],
            len: Len::new(),
            rep_len: Len::new(),
            literal,
        }
    }
}
