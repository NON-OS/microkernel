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

//! One alternate setting of an interface as the drivers bind on it.

use crate::bus::Pipes;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Interface {
    pub number: u8,
    pub alt: u8,
    pub class: u8,
    pub subclass: u8,
    pub protocol: u8,
    /// Its bulk pipes; an address of 0 is a direction it lacks.
    pub pipes: Pipes,
}

impl Interface {
    pub fn has_bulk_pair(&self) -> bool {
        self.pipes.bulk_in != 0 && self.pipes.bulk_out != 0
    }
}
