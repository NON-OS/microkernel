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

//! What goes ahead of grabs and focus, in this order:
//! 1. the press of a reserved key (Ctrl+Alt+Esc, the secure attention
//!    chord, the system keys) goes to the shell, even under a grab or a hold;
//! 2. a key's release goes where its press went when that was the shell, or
//!    while a hold is up (so a key held when a prompt came up is let go
//!    where it went down, and no window is left holding it);
//! 3. while the trusted band is held, every event goes to its holder: keys,
//!    clicks, motion and wheel, whatever a grab or the window manager says.
//!
//! Anything else routes as before.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    To(u32),
    Held(u32),
    Normal,
}

/// A key event's facts: a press or a release, whether it is reserved, and
/// for a release, where its press went.
#[derive(Clone, Copy, Debug)]
pub struct Key {
    pub down: bool,
    pub reserved: bool,
    pub pressed_in: Option<u32>,
}

pub fn preempt(key: Option<Key>, hold: Option<u32>, shell: u32) -> Route {
    if let Some(k) = key {
        if k.down && k.reserved && shell != 0 {
            return Route::To(shell);
        }
        match k.pressed_in {
            Some(p) if !k.down && p != 0 && (p == shell || hold.is_some()) => return Route::To(p),
            _ => {}
        }
    }
    hold.map_or(Route::Normal, Route::Held)
}
