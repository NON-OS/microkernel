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

//! The identifiers a disk is given, drawn from the caller's entropy.

use crate::guid::Guid;

/// The disk GUID, a unique GUID for each of the four partitions, and the
/// FAT volume id.
pub const ENTROPY_BYTES: usize = 16 + 4 * 16 + 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ids {
    pub disk: Guid,
    /// In `Region::ALL` order: store, plan, data, ESP.
    pub partitions: [Guid; 4],
    pub volume_id: u32,
}

impl Ids {
    pub fn from_entropy(e: &[u8; ENTROPY_BYTES]) -> Ids {
        let guid = |at: usize| {
            let mut bytes = [0u8; 16];
            bytes.copy_from_slice(&e[at..at + 16]);
            Guid::from_random(bytes)
        };
        Ids {
            disk: guid(0),
            partitions: [guid(16), guid(32), guid(48), guid(64)],
            volume_id: u32::from_le_bytes([e[80], e[81], e[82], e[83]]),
        }
    }
}
