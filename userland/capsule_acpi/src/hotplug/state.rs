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

//! What hot-plug remembers between checks: the functions each namespace
//! device stood for while it was there (a device that has left cannot be
//! asked), the buses each bridge led to, and ejects still waiting.

pub use super::stage::{Pending, Stage};
use alloc::vec::Vec;
use nonos_aml::namespace::Id;

pub type Bdf = (u16, u8, u8, u8);

pub struct Bridge {
    pub bdf: Bdf,
    pub secondary: u8,
    pub subordinate: u8,
}

#[derive(Default)]
pub struct Hotplug {
    pub known: Vec<(Id, Vec<Bdf>)>,
    pub bridges: Vec<Bridge>,
    pub pending: Vec<Pending>,
    /// Docks `_DCK(1)` has docked.
    pub docked: Vec<Id>,
    /// The ports native hot plug drives, when its switch is on.
    pub native: crate::hotplug::native::Native,
}

impl Hotplug {
    pub fn functions(&self, id: Id) -> &[Bdf] {
        self.known.iter().find(|k| k.0 == id).map_or(&[], |k| &k.1)
    }

    pub fn forget(&mut self, id: Id) {
        if let Some(i) = self.known.iter().position(|k| k.0 == id) {
            let (_, gone) = self.known.swap_remove(i);
            self.bridges.retain(|b| !gone.contains(&b.bdf));
        }
    }

    /// Every function on the buses below `bdf`, when it is a bridge.
    pub fn behind(&self, bdf: Bdf) -> Vec<Bdf> {
        let Some(b) = self.bridges.iter().find(|b| b.bdf == bdf) else { return Vec::new() };
        let (seg, lo, hi) = (bdf.0, b.secondary, b.subordinate.max(b.secondary));
        (lo..=hi)
            .flat_map(|bus| (0..32u8).flat_map(move |d| (0..8u8).map(move |f| (seg, bus, d, f))))
            .collect()
    }
}
