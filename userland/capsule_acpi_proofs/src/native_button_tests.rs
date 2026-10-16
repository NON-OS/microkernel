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

//! The attention button: an eject after `BUTTON_MS`, asked again every
//! `RETRY_MS` while a driver holds a function, cancelled by a second
//! press in either wait, and ignored on a slot without a button.

use super::native_slot_tests::{slot, PRESS, UP};
use crate::native::slot::{Act, Phase, Slot};
use crate::native::times::*;

#[test]
fn the_button_ejects_after_its_window_unless_pressed_again() {
    let mut s = slot(UP);
    s.step(PRESS, 0);
    assert_eq!(s.step(UP, BUTTON_MS - 1), Act::Nothing);
    assert_eq!(s.step(UP, BUTTON_MS), Act::Eject);
    assert_eq!(s.step(UP, BUTTON_MS + RETRY_MS - 1), Act::Nothing, "held: waits");
    assert_eq!(s.step(UP, BUTTON_MS + RETRY_MS), Act::Eject, "asked again");
    assert_eq!((s.ejected(), s.phase), (Act::PowerOff, Phase::Off));
    let mut c = slot(UP);
    c.step(PRESS, 0);
    assert_eq!((c.step(PRESS, 3_000), c.phase), (Act::Nothing, Phase::Occupied), "cancelled");
    assert_eq!(c.step(UP, BUTTON_MS + 1), Act::Nothing);
    let mut w = slot(UP);
    w.step(PRESS, 0);
    w.step(UP, BUTTON_MS);
    assert_eq!((w.step(PRESS, BUTTON_MS + 10), w.phase), (Act::Nothing, Phase::Occupied));
    let mut nb = Slot::new(false, true, UP);
    assert_eq!((nb.step(PRESS, 0), nb.phase), (Act::Nothing, Phase::Occupied), "no button");
}
