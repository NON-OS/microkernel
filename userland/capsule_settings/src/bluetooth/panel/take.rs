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

//! The service's replies taken into the panel. A reply the shared crate
//! does not read leaves the panel as it was, and says so with `false`.

use nonos_bt_proto::{Asked, Bonds, Health, Levels, Link};

use super::known::Panel;

impl Panel {
    pub fn took_health(&mut self, data: &[u8]) -> bool {
        Health::read(data).map(|h| self.health = Some(h)).is_some()
    }

    /// A window seen shut is no longer counted down.
    pub fn took_link(&mut self, data: &[u8]) -> bool {
        let Some(l) = Link::read(data) else { return false };
        if !l.has(Link::WINDOW) {
            self.window_from_ms = None;
        }
        self.link = Some(l);
        true
    }

    pub fn took_bonds(&mut self, data: &[u8]) -> bool {
        let Some(list) = Bonds::read(data) else { return false };
        for (i, slot) in self.bonds.iter_mut().enumerate() {
            *slot = list.get(i).unwrap_or_default();
        }
        self.bond_count = list.len();
        self.cursor = self.cursor.min(self.bond_count.saturating_sub(1));
        true
    }

    pub fn took_levels(&mut self, data: &[u8]) -> bool {
        let Some(list) = Levels::read(data) else { return false };
        for (i, slot) in self.levels.iter_mut().enumerate() {
            *slot = list.get(i).unwrap_or_default();
        }
        self.level_count = list.len();
        true
    }

    /// A new question wipes digits typed for the last one: they were for a
    /// pairing that has ended.
    pub fn took_asked(&mut self, data: &[u8]) -> bool {
        let Some(asked) = Asked::read(data) else { return false };
        if asked != self.asked {
            self.digits.clear();
        }
        self.asked = asked;
        true
    }
}
