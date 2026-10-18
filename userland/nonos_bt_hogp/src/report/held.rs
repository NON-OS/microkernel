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

//! Events from what a report says is held now against what the last said,
//! releases before presses, for reports laid out by a Report Map.

use super::input::Input;
use alloc::vec::Vec;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Held {
    keys: Vec<u8>,
}

impl Held {
    pub fn to(&mut self, now: Vec<u8>) -> Vec<Input> {
        let was = core::mem::replace(&mut self.keys, now);
        let ups = was.iter().filter(|k| !self.keys.contains(k)).map(|&k| Input::KeyUp(k));
        let downs = self.keys.iter().filter(|k| !was.contains(k)).map(|&k| Input::KeyDown(k));
        ups.chain(downs).collect()
    }

    /// Releases for everything held, for when the link goes.
    pub fn release_all(&mut self) -> Vec<Input> {
        self.to(Vec::new())
    }
}
