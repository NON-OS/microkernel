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
use super::store_erase::erase_slot;
use super::store_table::Table;
use super::store_write::write_payload;

/*
 * A withdrawal on a boot that keeps nothing. The kernel lets such a boot only
 * erase, so a record the disk already holds, at the length withdrawn, is
 * wiped where it lies: its payload zeroed first, then its descriptor, and the
 * next load reads the slot as empty. A name the disk does not hold stays off
 * it: nothing is ever appended or repointed, so no name, length or digest
 * reaches the disk from an amnesic boot.
 */
pub fn withdraw(name: &str, data: &[u8]) -> Result<(), BlkError> {
    if data.iter().any(|b| *b != 0) {
        return Err(BlkError::Inval);
    }
    let t = Table::read()?;
    let Some(entry) = t.entries.iter().find(|e| e.name == name) else {
        return Ok(());
    };
    if entry.len != data.len() as u64 {
        return Err(BlkError::Exists);
    }
    write_payload(entry.offset, &alloc::vec![0u8; data.len()])?;
    erase_slot(&t.toc, entry.slot)
}
