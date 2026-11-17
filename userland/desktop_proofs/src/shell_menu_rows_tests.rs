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

//! The right-click menu marks Delete, and only Delete, as the danger row: on
//! an item, Open and Rename stay plain, and nothing on the empty desktop is.

use nonos_toolkit::widgets::Item;

use crate::shell_menu_rows::{EMPTY_ITEMS, ITEM_ITEMS};

/// The indices of the danger rows among `items`. Making a danger row a
/// danger row again leaves it as it is, and changes any other row.
fn dangers(items: &[Item<'static>]) -> Vec<usize> {
    (0..items.len()).filter(|&i| items[i].danger() == items[i]).collect()
}

#[test]
fn only_delete_is_the_danger_row() {
    assert_eq!(ITEM_ITEMS[2], Item::new("Delete").danger(), "Delete is the danger row");
    assert_eq!(dangers(&ITEM_ITEMS), [2], "only Delete on an item");
    assert_eq!(dangers(&EMPTY_ITEMS), [] as [usize; 0], "none on the empty desktop");
}
