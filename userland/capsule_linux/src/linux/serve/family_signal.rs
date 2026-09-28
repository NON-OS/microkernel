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

//! Signals between processes of the family, settled after every answer: the
//! outbox is routed, and each process's parked threads take what now reaches
//! them.

use super::family::Family;

impl Family {
    pub(super) fn settle_signals(&mut self) {
        self.route_outbox();
        self.guests.iter_mut().for_each(super::deliver_wait::settle);
    }
}
