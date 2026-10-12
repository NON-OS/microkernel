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

//! Each Battery Level in turn, read once (Vol 3, Part G, 4.8.1); then, if
//! it can notify, its CCCD is looked for (`notify`).

use super::charge::Charge;
use super::state::Host;
use super::step::Step;
use super::types::Battery;
use crate::uuids::prop;
use nonos_bt_wire::att;

impl Host {
    pub(super) fn read_cell(&mut self, cell: usize, now_ms: u64) -> Step {
        let Some(c) = self.cells.get(cell) else {
            self.charge = Charge::Done;
            return Step::default();
        };
        let value = c.value;
        self.charge = Charge::Read { cell };
        self.ask(att::read(value), now_ms)
    }

    pub(super) fn on_read(&mut self, cell: usize, p: &[u8], now_ms: u64) -> Step {
        let battery = self.level(cell, p);
        let c = self.cells[cell];
        let next = c.value.saturating_add(1);
        let mut step = if c.props & prop::NOTIFY == 0 || next > c.end || c.value == 0xffff {
            self.read_cell(cell + 1, now_ms)
        } else {
            self.charge = Charge::Descs { cell, next };
            self.ask(att::find_information(next, c.end), now_ms)
        };
        step.battery = battery;
        step
    }

    /// A level's value: one octet, a percentage (BAS 1.0, 3.1). Anything
    /// else is not a level and is passed over.
    pub(super) fn level(&self, cell: usize, value: &[u8]) -> Option<Battery> {
        let &[percent] = value else { return None };
        let instance = u8::try_from(cell).ok()?;
        (percent <= 100).then_some(Battery { instance, percent })
    }
}
