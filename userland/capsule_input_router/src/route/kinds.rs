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

//! What kind of event this is, and a key event's facts for `preempt`.

use nonos_libc::{
    InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_KEY_DOWN,
    INPUT_KIND_KEY_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL, INPUT_KIND_TOUCH,
    INPUT_KIND_WHEEL,
};

use crate::state::Context;

use super::preempt::Key;
use super::{chord, shell_keys};

pub(super) fn key_facts(ctx: &Context, event: &InputEvent) -> Key {
    let (kind, code, flags) = (event.kind, event.code, event.flags);
    let reserved = chord::is_reserved_chord(kind, code, flags)
        || chord::is_secure_attention(kind, code, flags)
        || shell_keys::is_shell_key(code);
    let down = kind == INPUT_KIND_KEY_DOWN;
    Key { down, reserved, pressed_in: (!down).then(|| ctx.key_targets.held_by(code)).flatten() }
}

pub(super) fn is_keyboard(kind: u16) -> bool {
    kind == INPUT_KIND_KEY_DOWN || kind == INPUT_KIND_KEY_UP
}

pub(super) fn is_pointer(kind: u16) -> bool {
    matches!(
        kind,
        INPUT_KIND_POINTER_REL
            | INPUT_KIND_POINTER_ABS
            | INPUT_KIND_WHEEL
            | INPUT_KIND_BUTTON_DOWN
            | INPUT_KIND_BUTTON_UP
            | INPUT_KIND_TOUCH
    )
}
