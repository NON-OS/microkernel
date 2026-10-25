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

//! The supply voltages and their Power Control select bits.

use super::super::regs::{POWER_180, POWER_300, POWER_330};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vdd {
    V18,
    V30,
    V33,
}

impl Vdd {
    /// The Power Control voltage select bits, SD Bus Power clear.
    pub const fn select(self) -> u8 {
        match self {
            Vdd::V18 => POWER_180,
            Vdd::V30 => POWER_300,
            Vdd::V33 => POWER_330,
        }
    }

    pub const fn millivolts(self) -> u32 {
        match self {
            Vdd::V18 => 1800,
            Vdd::V30 => 3000,
            Vdd::V33 => 3300,
        }
    }
}
