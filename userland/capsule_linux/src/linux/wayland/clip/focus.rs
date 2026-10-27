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

//! Focus, as the selections see it: the input that shows the window has
//! it, and the keyboard entering a surface, before which Wayland has the
//! selections told again.

use nonos_app_skeleton::input::InputKind;

use crate::linux::call::family_ms;
use crate::linux::guest::Guest;

use super::offer::offer_all;
use super::which::Which;

/// A key or a button the window was given, which the router sends only
/// while it has focus (desk.rs).
pub fn input(guest: &mut Guest, kind: InputKind) {
    use InputKind::*;
    if matches!(kind, KeyDown | KeyUp | ButtonDown | ButtonUp) {
        guest.scene.clip.input_ms = family_ms().max(1);
    }
}

/// Just before wl_keyboard.enter: each selection is offered to every device
/// again, the desktop's when the guest holds no clipboard selection.
pub fn entered(guest: &mut Guest) {
    for which in [Which::Data, Which::Primary] {
        let source = guest.scene.clip.held[which.at()].or_else(|| super::desk::fallback(which));
        let (to, clip) = (&mut guest.display.to_client, &mut guest.scene.clip);
        offer_all(&mut guest.objects, clip, which, source, to);
    }
}
