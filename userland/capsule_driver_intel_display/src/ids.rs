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

//! Which Intel graphics functions this driver takes: the Gen9 device ids
//! in the data file, built into a table when the crate compiles.

use crate::ids_parse::{parse, Table};

pub const VENDOR_INTEL: u16 = 0x8086;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Skylake,
    KabyLake,
    CoffeeLake,
    CometLake,
}

static TABLE: Table = parse(include_bytes!("../data/gen9_ids.txt"));

/// The family of an Intel graphics device id this driver covers.
pub fn family(vendor: u16, device: u16) -> Option<Family> {
    if vendor != VENDOR_INTEL {
        return None;
    }
    TABLE.ids[..TABLE.len].iter().find(|e| e.0 == device).map(|e| e.1)
}

/// Every id in the table, in the file's order.
pub fn all() -> &'static [(u16, Family)] {
    &TABLE.ids[..TABLE.len]
}
