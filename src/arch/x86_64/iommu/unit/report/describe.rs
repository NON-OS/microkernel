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

//! One line describing the unit as the hardware reports it. Facts only. What
//! the kernel then does about them is bring-up's line to print, not this one,
//! because this code runs before anything has been decided.

use super::super::probe::UnitInfo;
use crate::sys::serial::Line;

pub(super) fn unit(count: usize, info: &UnitInfo) {
    let mut line = Line::new();
    line.str(b"[VT-D] units=").hex(count as u64);
    line.str(b" base=").hex(info.unit.base_pa());
    line.str(b" ver=").hex(info.version as u64);
    line.str(b" domains=").hex(info.domains as u64);
    line.str(b" gaw=").hex(info.max_address_width as u64);
    line.str(b" levels=").hex(info.levels.page_table_levels() as u64);
    if info.caching_mode {
        line.str(b" cm");
    }
    if info.requires_write_buffer_flush {
        line.str(b" rwbf");
    }
    if info.translation_enabled {
        // Firmware left it on with its own tables. The report turns it off
        // right after this line, before anything is programmed.
        line.str(b" te=firmware");
    }
    line.end();
}
