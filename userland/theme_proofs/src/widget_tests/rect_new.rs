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

//! `Rect::new` places the fields it is given, and `Rect::default` is the empty
//! rect at the origin.

use nonos_toolkit::widgets::Rect;

#[test]
fn rect_new_sets_each_field() {
    const R: Rect = Rect::new(1, 2, 3, 4);
    assert_eq!(R, Rect { x: 1, y: 2, w: 3, h: 4 });
}

#[test]
fn rect_default_is_all_zeros() {
    assert_eq!(Rect::default(), Rect { x: 0, y: 0, w: 0, h: 0 });
}
