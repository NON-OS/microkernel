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

//! What every request works on: the controller's ports, the clock, the
//! schedule memory, and the slot table.

use crate::devices::table::Table;
use crate::env::{Clock, Io, Mem};

pub struct Ctx<I: Io, C: Clock, M: Mem> {
    pub io: I,
    pub clock: C,
    pub mem: M,
    pub table: Table,
}

impl<I: Io, C: Clock, M: Mem> Ctx<I, C, M> {
    pub fn new(io: I, clock: C, mem: M) -> Self {
        Ctx { io, clock, mem, table: Table::default() }
    }
}
