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

//! What a sector holds, in the words a read-back failure is reported in.

use nonos_disk_map::KEY_LBA;

use super::plan::Layout;
use super::region::Region;
use crate::gpt::FIRST_USABLE_LBA;

impl Layout {
    pub fn what_is_at(&self, lba: u64) -> &'static str {
        match lba {
            0 => "the protective MBR",
            1 => "the GPT header",
            l if l < FIRST_USABLE_LBA => "the GPT entries",
            KEY_LBA => "the key header",
            l if l == self.backup_header_lba => "the backup GPT header",
            l if l >= self.backup_array_lba => "the backup GPT entries",
            l => Region::ALL
                .into_iter()
                .find(|r| self.extent(*r).contains(l))
                .map_or("a sector outside every partition", Region::what),
        }
    }
}
