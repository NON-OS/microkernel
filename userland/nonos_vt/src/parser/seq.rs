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

//! A control sequence as it was collected: private marker, parameters,
//! intermediate bytes and the final byte that names it.

use crate::limits::MAX_INTERMEDIATES;
use crate::params::Params;

#[derive(Clone, Default, Debug)]
pub struct Seq {
    /// `?`, `>`, `=` or `<` straight after the introducer, else 0.
    pub private: u8,
    pub params: Params,
    inter: [u8; MAX_INTERMEDIATES],
    ninter: usize,
    /// More intermediates than fit; the sequence is ignored.
    pub overflow: bool,
    pub final_byte: u8,
}

impl Seq {
    pub fn clear(&mut self) {
        self.private = 0;
        self.params.clear();
        self.ninter = 0;
        self.overflow = false;
        self.final_byte = 0;
    }

    pub fn collect(&mut self, b: u8) {
        if self.ninter < MAX_INTERMEDIATES {
            self.inter[self.ninter] = b;
            self.ninter += 1;
        } else {
            self.overflow = true;
        }
    }

    pub fn inter(&self) -> &[u8] {
        &self.inter[..self.ninter]
    }

    /// Whether the sequence carries anything it could not hold whole.
    pub fn truncated(&self) -> bool {
        self.overflow || self.params.overflow
    }
}
