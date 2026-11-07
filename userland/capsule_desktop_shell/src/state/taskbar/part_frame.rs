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

//! The areas a frame that changed part of the screen composes. The shell
//! draws its chrome whole every frame, toasts and all, so toasts raised or
//! gone since the last frame are on this one: their area is composed with
//! the change, not the whole screen. A dock shown or hidden since it was
//! drawn, or the lock screen over everything, still has the frame shown whole.

use super::focus_parts::Area;

/// The most areas one frame names: the change's two strips and the toasts.
pub const MAX_PARTS: usize = 3;

/// Areas in the order they are shown; the first is the one the frame is
/// presented with, and every one is committed after it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parts {
    areas: [Area; MAX_PARTS],
    len: usize,
}

impl Parts {
    pub fn as_slice(&self) -> &[Area] {
        &self.areas[..self.len]
    }

    /// The area the frame is presented with.
    pub fn first(&self) -> Area {
        self.areas[0]
    }
}

/// `changed`, then `toasts` (the toasts' area, when toasts not yet drawn go
/// on this frame). None when the frame is to be shown whole: the dock owed a
/// show or hide, the lock screen up, nothing named, or more than MAX_PARTS.
pub fn part_frame(
    changed: &[Area],
    toasts: Option<Area>,
    dock_owed: bool,
    locked: bool,
) -> Option<Parts> {
    if dock_owed || locked || changed.is_empty() {
        return None;
    }
    let mut parts = Parts { areas: [(0, 0, 0, 0); MAX_PARTS], len: 0 };
    for &a in changed.iter().chain(toasts.iter()) {
        *parts.areas.get_mut(parts.len)? = a;
        parts.len += 1;
    }
    Some(parts)
}
