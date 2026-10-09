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

//! The bits inside `Prefs::rails`.
//!
//! One byte of the preferences record, named here rather than written as a
//! literal at the places that read it.
//!
//! Bit 0 was meant to turn the telemetry monitor off, but nothing ever set or
//! read it: the rail samples whenever the window ticks. The codec still keeps
//! the bit (`RAILS_MASK`), so a record that carries it reads back unchanged;
//! a switch for the monitor would take it.

/// Bit 1: the left rail is on screen.
///
/// Off by default. Ctrl-B brings it in, the same key that shows and hides the
/// file tree in the editor, so one gesture uncovers the side panel anywhere in
/// the system.
pub const RAIL_VISIBLE: u8 = 0b10;
