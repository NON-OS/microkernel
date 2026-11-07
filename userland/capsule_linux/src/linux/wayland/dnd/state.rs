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

//! What the server keeps of a guest's drags. Pure, so the host proofs hold
//! it.

use alloc::vec::Vec;

#[derive(Default)]
pub struct Dnd {
    /// The actions each of the guest's sources offers (set_actions).
    pub actions: Vec<(u32, u32)>,
    /// Whether a pointer button is held: a drag starts only during one.
    pub pressed: bool,
    /// The press held was kept through the keyboard leaving the window
    /// (input/focus.rs): at its release its serial goes too, unless the
    /// guest has the keyboard again.
    pub kept: bool,
    pub drag: Option<Drag>,
}

pub struct Drag {
    /// The data source; None for a drag within one client, offering nothing.
    pub source: Option<u32>,
    /// The surface the drag is over, and the offer each device was given
    /// there.
    pub over: Option<u32>,
    pub offers: Vec<(u32, u32)>,
    /// Whether the destination accepted a mime type, the actions it takes
    /// and prefers, and the action chosen.
    pub accepted: bool,
    pub dest: (u32, u32),
    pub action: u32,
    /// Dropped: the pointer is free again, and the destination reads and
    /// finishes.
    pub dropped: bool,
}

impl Dnd {
    /// The actions `source` offers; none set, every one.
    pub fn offered(&self, source: Option<u32>) -> u32 {
        let set = source.and_then(|s| self.actions.iter().find(|a| a.0 == s));
        set.map_or(super::action::ALL, |a| a.1)
    }

    /// A drag that holds the pointer.
    pub fn dragging(&self) -> bool {
        self.drag.as_ref().is_some_and(|d| !d.dropped)
    }

    /// Whether `offer` is the drag's.
    pub fn offer_of(&self, offer: u32) -> bool {
        self.drag.as_ref().is_some_and(|d| d.offers.iter().any(|o| o.1 == offer))
    }
}
