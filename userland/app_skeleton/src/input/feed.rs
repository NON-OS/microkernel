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

//! Passes the pointer events of a window to the toolkit's `Controls`.

use nonos_toolkit::widgets::{Controls, Step};

use super::event::InputEvent;
use super::kind::InputKind;

/// Feeds one event to `controls` and returns what it asks of the app. The
/// input router says the pointer left with a move to (-1, -1), which lands
/// left of and above the content. Keys, wheel turns and relative moves are
/// `Step::Idle`; the app handles them itself.
pub fn feed<A: Copy + PartialEq>(controls: &mut Controls<A>, ev: &InputEvent) -> Step<A> {
    match ev.kind {
        InputKind::PointerAbs if ev.x < 0 && ev.y < 0 => controls.left(),
        InputKind::PointerAbs => controls.moved(ev.x, ev.y),
        InputKind::ButtonDown => controls.pressed(ev.x, ev.y),
        InputKind::Touch => controls.touched(ev.x, ev.y),
        InputKind::ButtonUp => controls.released(),
        _ => Step::Idle,
    }
}
