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

//! The model PPM's GET_PDOS: the partner's source PDOs from the offset
//! asked, as many as asked and it has, zero past the end; an error for a
//! connector it lacks, for anything but partner source PDOs, or when told
//! to refuse.

use crate::model::Model;
use crate::model_reply::{CC, ERROR};

impl Model {
    pub(crate) fn pdos_reply(&mut self, control: u64) -> u32 {
        let f = control >> 16;
        let (n, partner, source) = ((f & 0x7F) as usize, f >> 7 & 1 == 1, f >> 18 & 1 == 1);
        let (offset, count) = ((f >> 8 & 0xFF) as usize, (f >> 16 & 3) as usize + 1);
        if self.refuse_pdos || !partner || !source || !(1..=self.offers.len()).contains(&n) {
            return CC | ERROR;
        }
        let offers = &self.offers[n - 1];
        for i in 0..count {
            let w = offers.get(offset + i).copied().unwrap_or(0);
            self.message[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
        }
        CC | ((count * 4) as u32) << 8
    }
}
