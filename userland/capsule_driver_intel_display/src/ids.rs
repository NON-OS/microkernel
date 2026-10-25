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

//! Which Intel graphics functions this driver takes: the device ids in
//! one data file per PRM generation, built into tables when the crate
//! compiles. An id in two files stops nothing here; the proofs refuse it.

pub use crate::family::Family;
use crate::ids_parse::{parse, Table};

pub const VENDOR_INTEL: u16 = 0x8086;

static TABLES: [Table; 5] = [
    parse(include_bytes!("../data/gen7_5_ids.txt")),
    parse(include_bytes!("../data/gen8_ids.txt")),
    parse(include_bytes!("../data/gen9_ids.txt")),
    parse(include_bytes!("../data/gen11_ids.txt")),
    parse(include_bytes!("../data/gen12_ids.txt")),
];

/// The family of an Intel graphics device id this driver covers.
pub fn family(vendor: u16, device: u16) -> Option<Family> {
    if vendor != VENDOR_INTEL {
        return None;
    }
    all().find(|e| e.0 == device).map(|e| e.1)
}

/// Every id, file by file in the files' order.
pub fn all() -> impl Iterator<Item = (u16, Family)> {
    TABLES.iter().flat_map(|t| t.ids[..t.len].iter().copied())
}
