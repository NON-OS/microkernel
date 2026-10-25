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

//! The boot's one-line summaries of the embedded controller and of the
//! thermal zones and fans.

use crate::cooling::Cooling;
use crate::ec::Ec;
use crate::sys::log::Line;

pub(super) fn say_ec(ec: Option<&Ec>) {
    match ec {
        Some(e) => Line::new(b"EC data=")
            .hex(e.ports.data as u64)
            .str(b" command=")
            .hex(e.ports.command as u64)
            .str(b" queries=")
            .dec(e.queries.len() as u64)
            .end(),
        None => Line::new(b"no embedded controller").end(),
    }
}

pub(super) fn say_cooling(c: &Cooling) {
    Line::new(b"thermal zones=")
        .dec(c.zones.len() as u64)
        .str(b" fans=")
        .dec(c.fans.len() as u64)
        .end();
}
