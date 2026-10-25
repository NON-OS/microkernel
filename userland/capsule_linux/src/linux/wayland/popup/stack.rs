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

//! What the server keeps of a client's popups. Pure, so the host proofs
//! hold it; the rules a grab is held to are rules.rs.

use alloc::vec::Vec;

use super::positioner::Positioner;

pub struct Popup {
    /// The xdg_popup.
    pub id: u32,
    pub surface: u32,
    /// The parent's surface.
    pub parent: u32,
    /// Where it was placed, on the toplevel's surface.
    pub rect: (i32, i32, i32, i32),
    pub grabbed: bool,
}

#[derive(Default)]
pub struct Popups {
    /// Grabbing popups bottom first, then those still waiting for a grab.
    pub list: Vec<Popup>,
    pub positioners: Vec<(u32, Positioner)>,
    /// The serial of the latest press, key or button, this client was sent.
    pub last_press: Option<u32>,
}

impl Popups {
    /// The topmost grabbing popup.
    pub fn top(&self) -> Option<&Popup> {
        self.list.iter().rev().find(|p| p.grabbed)
    }
}
