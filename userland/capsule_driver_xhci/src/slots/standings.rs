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

//! Each judged configuration's standing, by slot, the slot's current one
//! last, and the held ones listed for the owner's answer (OP_USB_HELD).

use super::authorize::{Authority, Standing, DEFAULT_POLICY};

impl Default for Authority {
    fn default() -> Self {
        Self::new(DEFAULT_POLICY)
    }
}

impl Authority {
    /// The standing of the slot's current configuration.
    pub fn standing(&self, slot: u8) -> Option<Standing> {
        self.list.iter().rev().find(|e| e.0 == slot).map(|e| e.2)
    }

    /// Configuration `value`'s standing, made the slot's current one.
    pub(super) fn current(&mut self, slot: u8, value: u8) -> Option<Standing> {
        let at = self.list.iter().position(|e| e.0 == slot && e.1 == value)?;
        let e = self.list.remove(at);
        self.list.push(e);
        Some(e.2)
    }

    pub(super) fn set(&mut self, slot: u8, value: u8, standing: Standing) {
        self.list.retain(|e| e.0 != slot || e.1 != value);
        self.list.push((slot, value, standing));
    }

    pub fn forget(&mut self, slot: u8) {
        self.list.retain(|e| e.0 != slot);
    }

    pub fn held(&self) -> impl Iterator<Item = (u8, u32)> + '_ {
        self.list.iter().filter_map(|e| match e.2 {
            Standing::Held { ticket } => Some((e.0, ticket)),
            _ => None,
        })
    }
}
