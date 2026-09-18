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
//! Two bits, both stored in one byte of the preferences record, so they are
//! named here rather than written as literals at the places that read them.

/// Bit 0, inverted: the telemetry monitor is running.
///
/// Stored inverted because the default record is all zeroes and the monitor
/// should be on for anyone who has asked for the rail at all.
pub const RAIL_MONITOR_OFF: u8 = 0b01;

/// Bit 1: the left rail is on screen.
///
/// Off by default. Ctrl-B brings it in, the same key that shows and hides the
/// file tree in the editor, so one gesture uncovers the side panel anywhere in
/// the system.
pub const RAIL_VISIBLE: u8 = 0b10;
