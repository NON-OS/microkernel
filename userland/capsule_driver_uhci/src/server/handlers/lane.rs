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

//! Whose a root port's polled place is. No hub is enumerated (hub ops
//! answer E_INVAL), and a port is addressed by one slot at a time, so a
//! second slot polling on a port cannot arise; were it to, it is refused,
//! never given the place another slot's armed transfer holds.

use crate::devices::table::Table;

/// Whether a slot other than `slot` has its polled IN armed on `port`.
pub fn taken(t: &Table, slot: u8, port: u8) -> bool {
    let mut others = t.devs.iter().enumerate().filter(|(j, _)| j + 1 != slot as usize);
    others.any(|(_, d)| d.is_some_and(|d| d.port == port && d.poll.is_some()))
}
