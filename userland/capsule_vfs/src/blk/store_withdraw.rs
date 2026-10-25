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
//! Withdrawing a record on a boot that keeps nothing.

use super::error::BlkError;
use super::store_table::Table;
use super::store_write::same_bytes;

/*
 * A withdrawal on a boot that keeps nothing. The zeros may only take the place
 * of a record the disk already holds, at its own length, through the same
 * replace a kept record goes through; a name the disk does not hold stays off
 * it. Nothing is ever appended, so no name, length or digest reaches the disk
 * from an amnesic boot.
 */
pub fn withdraw(name: &str, data: &[u8]) -> Result<(), BlkError> {
    if data.iter().any(|b| *b != 0) {
        return Err(BlkError::Inval);
    }
    let t = Table::read()?;
    let Some(index) = t.entries.iter().position(|e| e.name == name) else {
        return Ok(());
    };
    same_bytes(&t.entries[index], data).or_else(|_| {
        super::store_replace::replace(&t.toc, index, &t.entries, t.floor, t.capacity_bytes, data)
    })
}
