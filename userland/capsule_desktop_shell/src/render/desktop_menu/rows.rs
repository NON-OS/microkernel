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

//! The right-click menu's rows, apart from where the menu opens.

use nonos_toolkit::widgets::Item;

/// Rows for the empty-desktop menu and the per-item menu. Row index doubles as
/// the action selector, matching the order the input handler switches on.
/// Delete removes the item, so it is the danger row.
pub const EMPTY_ITEMS: [Item<'static>; 2] = [Item::new("New Folder"), Item::new("New File")];
pub const ITEM_ITEMS: [Item<'static>; 3] =
    [Item::new("Open"), Item::new("Rename"), Item::new("Delete").danger()];
