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

//! DC_STATE_EN in the model: a change of the DC state field reads back
//! two reads later, as the hardware leaves or enters the state, and
//! never when `dead`.

use super::gen9::Gen9;
use crate::engine::dc::{DC_MASK, DC_STATE_EN};

impl Gen9 {
    pub(super) fn dc_write(&mut self, off: u32, v: u32, old: u32) {
        if off != DC_STATE_EN || (v ^ old) & DC_MASK == 0 {
            return;
        }
        self.regs.insert(off, (v & !DC_MASK) | (old & DC_MASK));
        if !self.dead {
            self.pending.push((2, off, v));
        }
    }
}
