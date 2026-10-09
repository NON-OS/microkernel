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

use nonos_libc::{
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_REL, INPUT_KIND_WHEEL,
};

use super::button_changes::button_changes;
use super::mouse_event::MouseEvent;
use super::mouse_report::MOUSE_BUTTONS;
use super::post_wire::send;

pub fn publish(ev: MouseEvent, previous_buttons: u8) -> bool {
    let mut ok = true;
    if ev.dx != 0 || ev.dy != 0 {
        ok &= send(INPUT_KIND_POINTER_REL, 0, 0, ev.dx as i32, ev.dy as i32);
    }
    if ev.dz != 0 {
        ok &= send(INPUT_KIND_WHEEL, 0, 0, 0, ev.dz as i32);
    }
    ok & publish_buttons(previous_buttons, ev.buttons)
}

fn publish_buttons(previous: u8, current: u8) -> bool {
    let mut ok = true;
    button_changes(previous, current, MOUSE_BUTTONS, |button, down| {
        let kind = if down { INPUT_KIND_BUTTON_DOWN } else { INPUT_KIND_BUTTON_UP };
        ok &= send(kind, 0, button, 0, 0);
    });
    ok
}
