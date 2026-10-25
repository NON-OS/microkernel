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

//! What a believed GPT header says, as the reader keeps it.

use super::array::array_sectors;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Header {
    pub(crate) disk_guid: [u8; 16],
    pub(crate) first_usable: u64,
    pub(crate) last_usable: u64,
    pub(crate) entries_lba: u64,
    pub(crate) entries: u32,
    pub(crate) entry_size: u32,
    pub(crate) entries_crc: u32,
}

impl Header {
    pub(crate) fn array_sectors(&self) -> u64 {
        array_sectors(self.entries, self.entry_size)
    }
}
