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

//! The lid rule alone: a repeated word changes nothing, and the pointer is
//! hidden on the cursor plane while the lid is closed.

use crate::state::cursor::CursorState;
use crate::state::lid_rule::Lid;

#[test]
fn saying_what_the_lid_already_is_changes_nothing() {
    assert_eq!(Lid::Open.moved(false), (Lid::Open, false));
    assert_eq!(Lid::Dark.moved(true), (Lid::Dark, false));
    assert_eq!(Lid::Darkening.moved(true), (Lid::Darkening, false));
    assert_eq!(Lid::Darkening.moved(false), (Lid::Open, true));
    let p = CursorState { x: 5, y: 6, visible: true };
    assert!(!Lid::Darkening.pointer(p).visible && !Lid::Dark.pointer(p).visible);
    assert!(Lid::Open.pointer(p) == p);
}
