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

//! The rows as Rust, for src/vendor/table.rs to include.

use std::fmt::Write;

/// Store path, program BLAKE3, policy, destination list (empty for none).
pub type Row = (String, [u8; 32], [u8; crate::layout::LEN], Vec<u8>);

pub fn source(rows: &[Row]) -> String {
    let mut out = String::from("&[\n");
    for (store_path, program_blake3, policy, dests) in rows {
        let _ = writeln!(
            out,
            "    VendorRow {{ store_path: {store_path:?}, program_blake3: {program_blake3:?}, \
             policy: {policy:?}, dests: &{dests:?} }},"
        );
    }
    out.push(']');
    out
}
