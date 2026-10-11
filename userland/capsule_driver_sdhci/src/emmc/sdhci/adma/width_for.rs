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

//! The addressing width a transfer needs.

use super::Width;

/// The width a transfer needs: 32-bit when every byte of the table and the
/// data lies below 4 GiB, else 64-bit if the host has it, else none.
pub fn width_for(
    table_bus: u64,
    table_len: u64,
    data_bus: u64,
    data_len: u64,
    dma64: bool,
) -> Option<Width> {
    let low = |a: u64, n: u64| a.checked_add(n).is_some_and(|end| end <= 1 << 32);
    if low(table_bus, table_len) && low(data_bus, data_len) {
        return Some(Width::A32);
    }
    if dma64 {
        return Some(Width::A64);
    }
    None
}
