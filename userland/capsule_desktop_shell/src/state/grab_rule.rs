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

//! What the shell holds of the input router's grabs, and how it moves from one
//! holding to the next.
//!
//! Every menu, the Launchpad, a dialog and an icon drag is drawn in the shell's
//! chrome band, over every window, and a press while one is up belongs to it:
//! a press outside a menu closes the menu, it does not click the window drawn
//! under it. The shell holds a pointer grab for as long as any of them is up,
//! and a key grab while keys edit its own text (a rename, the Launchpad's
//! search) or answer what it shows: Tab, Enter and Esc on a dialog, and Esc
//! on an open menu, would otherwise reach the window under it. The router releases a holder's keyboard and pointer grabs
//! together, so dropping one while keeping the other is a release and a fresh
//! request.

/// The router's kinds, as bits of a grab mask (nonos_libc INPUT_KIND_*).
pub const KEY_DOWN_BIT: u32 = 1 << 0;
/// Absolute motion, the wheel, presses, releases and touches. Relative
/// motion is left out: the router turns it into absolute motion for the shell
/// on its own, and leaves relative devices' deltas to whoever asks for them.
pub const POINTER_BITS: u32 = (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7);

/// What is up on the desktop that wants input to itself.
#[derive(Clone, Copy, Default)]
pub struct Modal {
    pub drag: bool,
    pub launchpad: bool,
    pub menu: bool,
    pub dialog: bool,
    pub rename: bool,
}

pub fn wanted(m: Modal) -> u32 {
    let mut mask = 0;
    if m.rename || m.launchpad || m.menu || m.dialog {
        mask |= KEY_DOWN_BIT;
    }
    if m.drag || m.launchpad || m.menu || m.dialog {
        mask |= POINTER_BITS;
    }
    mask
}

/// The calls that take the router from `held` to `want`: whether to release
/// first, then the mask to request, if any.
pub fn steps(held: u32, want: u32) -> (bool, Option<u32>) {
    if held == want {
        return (false, None);
    }
    if want == 0 {
        return (true, None);
    }
    (held & !want != 0, Some(want))
}
