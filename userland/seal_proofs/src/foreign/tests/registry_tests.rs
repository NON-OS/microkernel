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

//! Removing a guest's row, as enrol does when its supervisor died meanwhile,
//! against drop_rows, which also orphans the guests a pid hosted.

use crate::foreign::registry::{guest_count, insert, is_foreign, remove, supervisor_of};

#[test]
fn removing_a_guest_drops_its_row_and_no_other() {
    let (sup, a, b) = (9104, 9105, 9106);
    assert!(insert(a, sup) && insert(b, sup));
    remove(a);
    assert!(!is_foreign(a));
    assert_eq!((guest_count(sup), supervisor_of(b)), (1, Some(sup)));
    remove(sup);
    assert_eq!(supervisor_of(b), Some(sup), "sup was no guest: b is not orphaned");
    remove(b);
    assert_eq!(guest_count(sup), 0);
}
