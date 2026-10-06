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

//! The position weights a consensus publishes, and a safe default.

use super::super::relay::Flags;

/// The position weights a consensus publishes, in units of ten thousand.
#[derive(Clone, Copy)]
pub struct Weights {
    pub wgg: u32,
    pub wgd: u32,
    pub wmg: u32,
    pub wmd: u32,
    pub wme: u32,
    pub wmm: u32,
    pub wee: u32,
    pub wed: u32,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            wgg: 10_000,
            wgd: 10_000,
            wmg: 10_000,
            wmd: 10_000,
            wme: 10_000,
            wmm: 10_000,
            wee: 10_000,
            wed: 10_000,
        }
    }
}

/// Which position a relay is being weighed for.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Guard,
    Middle,
    Exit,
}

pub(super) fn dual(flags: &Flags) -> bool {
    flags.guard && flags.exit
}
