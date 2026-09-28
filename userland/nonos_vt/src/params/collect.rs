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

//! Collecting parameters as their bytes arrive.

use crate::limits::{MAX_PARAM, MAX_PARAMS};

#[derive(Clone, Default, Debug)]
pub struct Params {
    pub(super) vals: [u16; MAX_PARAMS],
    /// Bit `i` set: value `i` followed a colon, a sub-parameter of the one
    /// before it.
    pub(super) sub: u32,
    pub(super) len: usize,
    /// More parameters than fit; the sequence is ignored.
    pub overflow: bool,
}

impl Params {
    pub fn clear(&mut self) {
        self.len = 0;
        self.sub = 0;
        self.overflow = false;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn digit(&mut self, d: u8) {
        if self.len == 0 {
            self.len = 1;
            self.vals[0] = 0;
        }
        let v = &mut self.vals[self.len - 1];
        *v = (*v as u32 * 10 + d as u32).min(MAX_PARAM as u32) as u16;
    }

    /// A separator: `colon` makes the next value a sub-parameter.
    pub fn separator(&mut self, colon: bool) {
        if self.len == 0 {
            self.len = 1;
            self.vals[0] = 0;
        }
        if self.len == MAX_PARAMS {
            self.overflow = true;
            return;
        }
        self.vals[self.len] = 0;
        if colon {
            self.sub |= 1 << self.len;
        }
        self.len += 1;
    }
}
