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

//! The line a working bring-up prints, "tg3: up <device id> rev <chip
//! revision> link=<up|down>", and the link state it records so the first
//! change after it is what the server logs next. The station address is not
//! printed: console lines outlive the session it was drawn for.

use crate::link::LinkState;
use crate::log::Line;
use crate::setup::Driver;

pub fn up(d: &mut Driver, state: LinkState) {
    d.link = Some(state);
    let mut line = Line::new();
    line.text("up ").hex(d.pci_device as u32, 4);
    line.text(" rev ").hex(d.chip.rev, 8);
    line.text(if state.up { " link=up" } else { " link=down" });
    line.send();
}
