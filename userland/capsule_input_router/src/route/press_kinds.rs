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

//! The events pressed and released: keys, and pad buttons, which follow
//! focus as keys do (`pad.rs`). Which kind releases which press, and each
//! one's entry in the press table.

use nonos_libc::{InputEvent, INPUT_KIND_KEY_UP};

use super::pad::{self, PAD_BUTTON_DOWN, PAD_BUTTON_UP};
use crate::state::key_targets::{key_of, Key};

/// A pad button: pressed and released like a key.
pub(super) fn is_pad_button(kind: u16) -> bool {
    kind == PAD_BUTTON_DOWN || kind == PAD_BUTTON_UP
}

pub(super) fn is_pad(kind: u16) -> bool {
    is_pad_button(kind) || kind == pad::PAD_AXIS
}

pub(super) fn is_release(kind: u16) -> bool {
    kind == INPUT_KIND_KEY_UP || kind == PAD_BUTTON_UP
}

/// The release that answers a press of this kind.
pub(super) fn release_of(kind: u16) -> u16 {
    if is_pad_button(kind) {
        PAD_BUTTON_UP
    } else {
        INPUT_KIND_KEY_UP
    }
}

/// An event's entry in the press table: a key by its source and code, a
/// pad button by its source, pad and button (`pad::slot`). A press on one
/// device is never released, nor taken over, by another's (K8b).
pub(super) fn slot(event: &InputEvent) -> Key {
    let code = if is_pad_button(event.kind) { pad::slot(event) } else { event.code };
    key_of(event.flags, code)
}
