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

//! The loader's boot media record (`BootMedia`, forty bytes) read for the
//! one thing the disk choice needs: the GUID of the GPT partition the loader
//! came from. An MBR partition, or no partition, names none.

/// Byte 4: the table; byte 5: what the signature is; 24..40: the signature.
const TABLE_GPT: u8 = 2;
const SIGNATURE_GUID: u8 = 2;
const RECORD_LEN: usize = 40;

pub(crate) fn boot_guid(record: &[u8]) -> Option<[u8; 16]> {
    if record.len() != RECORD_LEN || record[4] != TABLE_GPT || record[5] != SIGNATURE_GUID {
        return None;
    }
    let mut guid = [0u8; 16];
    guid.copy_from_slice(&record[24..40]);
    // An all-zero GUID is no partition at all, and would match unused slots.
    (guid != [0; 16]).then_some(guid)
}
