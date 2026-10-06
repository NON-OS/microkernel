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

//! One line when the link changes, "igc: link up 2500 full" or
//! "igc: link down", and nothing while it holds. The owner reads these to
//! see what the PHY negotiated without a tool on the machine.

use crate::log::Line;

use super::status::{changed, LinkState};

pub fn note(last: &mut Option<LinkState>, now: LinkState) {
    if !changed(*last, now) {
        return;
    }
    *last = Some(now);
    let mut line = Line::new();
    if now.up {
        line.text("link up ").dec(now.mbps as u32);
        line.text(if now.full { " full" } else { " half" });
    } else {
        line.text("link down");
    }
    line.send();
}
