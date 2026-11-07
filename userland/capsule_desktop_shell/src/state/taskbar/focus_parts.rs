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

//! What a change of the focused app changes on the screen: the menu bar,
//! whose first title names the app, and the dock's area, whose mark shows
//! it. Pure; held in desktop_proofs.
//!
//! Every focus that moved the named app repainted the shell whole and
//! committed the whole screen, so a press on the title bar of a window
//! behind another composed all 1920x1080 pixels, every layer, before the
//! drag it started could move. The two strips are composed again now and
//! nothing between them, unless the whole frame is owed for another reason
//! (the dock shown or hidden, toasts not yet drawn, the lock screen) or a
//! menu-bar drop-down is open, whose rows follow the focused app.

/// A part of the screen: x, y, width, height.
pub type Area = (u32, u32, u32, u32);

/// The parts to compose again, the bar first; None when the whole frame
/// is owed or a drop-down is open.
pub fn focus_parts(bar: Area, dock: Area, whole_owed: bool, menu_open: bool) -> Option<[Area; 2]> {
    (!whole_owed && !menu_open).then_some([bar, dock])
}
