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

//! Gamepads (B-57, usb_hid): a button's press and release and an axis's
//! move, with the pad's number in `y`. They follow focus exactly as keys
//! do: a press goes to the focused window, its release to where the press
//! went, an axis to the focused window; never to a window that is not
//! focused, and, ahead of all this, to the trusted band's holder while a
//! prompt is up (`preempt`). Before, every subscriber to the kind got them,
//! so a window in the background read the pad played in another.

use nonos_libc::InputEvent;

use crate::state::Context;

use super::deliver::deliver_one;
use super::key_deliver::deliver_key;

/// The kinds B-57 posts (its libc constants land with B's archive).
pub const PAD_BUTTON_DOWN: u16 = 8;
pub const PAD_BUTTON_UP: u16 = 9;
pub const PAD_AXIS: u16 = 10;

/// A pad button's entry in the press table, apart from every key code
/// (which stay under 0x8000_0000): the pad's number, then the button.
pub fn slot(event: &InputEvent) -> u32 {
    0x8000_0000 | ((event.y as u32 & 0x7FFF) << 16) | (event.code & 0xFFFF)
}

pub fn route_pad(ctx: &mut Context, event: &InputEvent) -> u32 {
    if event.kind == PAD_AXIS {
        let pid = super::focus_now::focus_now(ctx);
        let delivered =
            if ctx.subscriptions.allows(pid, event.kind) { deliver_one(pid, event) } else { 0 };
        ctx.record(delivered);
        return delivered;
    }
    let pressed_in = (event.kind == PAD_BUTTON_UP).then(|| ctx.key_targets.held_by(slot(event)));
    let pid = match pressed_in.flatten().filter(|&p| p != 0) {
        Some(p) => p,
        None => super::focus_now::focus_now(ctx),
    };
    deliver_key(ctx, pid, event)
}
