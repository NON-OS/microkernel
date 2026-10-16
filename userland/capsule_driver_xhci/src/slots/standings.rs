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

//! Each judged device's standing, by slot, and the held ones listed for
//! the owner's answer (OP_USB_HELD).

use super::authorize::{Authority, Standing, DEFAULT_POLICY};

impl Default for Authority {
    fn default() -> Self {
        Self::new(DEFAULT_POLICY)
    }
}

impl Authority {
    pub fn standing(&self, slot: u8) -> Option<Standing> {
        self.list.iter().find(|s| s.0 == slot).map(|s| s.1)
    }

    pub(super) fn set(&mut self, slot: u8, standing: Standing) {
        self.forget(slot);
        self.list.push((slot, standing));
    }

    pub fn forget(&mut self, slot: u8) {
        self.list.retain(|s| s.0 != slot);
    }

    pub fn held(&self) -> impl Iterator<Item = (u8, u32)> + '_ {
        self.list.iter().filter_map(|s| match s.1 {
            Standing::Held { ticket } => Some((s.0, ticket)),
            _ => None,
        })
    }
}
