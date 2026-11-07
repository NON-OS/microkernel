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

//! The two turns a client takes outside draw and present: the buffer the
//! layer was first submitted with, and a drawn buffer whose present the
//! compositor refused.

use super::super::swapchain_damage::Stale;
use super::{Chain, Misuse, Slot};

impl Chain {
    /// Buffer `i`, never drawn by this chain, is what the layer was first
    /// submitted with: shown from now on. Only while nothing is shown.
    pub fn adopt(&mut self, i: usize) -> Result<(), Misuse> {
        if self.slots[..self.count].iter().any(|s| *s != Slot::Free) {
            return Err(Misuse::AlreadyStarted);
        }
        self.slot(i).ok_or(Misuse::NoSuchBuffer)?;
        self.slots[i] = Slot::Shown;
        self.stale[i] = Stale::None;
        Ok(())
    }

    /// Buffer `i` was drawn but its present was refused: free again, and
    /// out of date as a whole, since what it holds was never shown.
    pub fn cancel(&mut self, i: usize) -> Result<(), Misuse> {
        match self.slot(i) {
            None => Err(Misuse::NoSuchBuffer),
            Some(Slot::Drawing) => {
                (self.slots[i], self.stale[i]) = (Slot::Free, Stale::Whole);
                Ok(())
            }
            Some(_) => Err(Misuse::NotDrawing),
        }
    }

    /// The buffer the compositor shows, if any.
    pub fn shown(&self) -> Option<usize> {
        self.slots[..self.count].iter().position(|s| *s == Slot::Shown)
    }
}
