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

//! The order GOP handles are tried in: the console's first, as Linux's
//! find_gop takes them, and the firmware's order kept within each group.

use crate::display::order::console_order;

#[test]
fn the_console_handle_is_tried_first() {
    // A two-GPU laptop whose firmware lists the dGPU with no panel first.
    assert_eq!(console_order(&[false, true]), vec![1, 0]);
    // OVMF with two displays: three handles, the last without ConsoleOut.
    assert_eq!(console_order(&[true, true, false]), vec![0, 1, 2]);
}

#[test]
fn firmware_order_is_kept_inside_each_group() {
    assert_eq!(console_order(&[false, true, false, true]), vec![1, 3, 0, 2]);
    assert_eq!(console_order(&[false, false, false]), vec![0, 1, 2]);
    assert_eq!(console_order(&[]), Vec::<usize>::new());
}
