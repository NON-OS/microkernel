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

use crate::constants::ata::LBA48_LIMIT;

/// The sectors `lba..lba + sectors` are at least one, lie inside a disk of
/// `capacity` sectors, and each has an LBA the FIS's 48 bits carry. The end
/// is summed checked, so a span that wraps u64 lies nowhere. Pure, so the
/// host proofs hold the same check `transfer` makes before it builds a
/// command, whoever asked for the span.
pub fn within(capacity: u64, lba: u64, sectors: u32) -> bool {
    let Some(end) = lba.checked_add(u64::from(sectors)) else {
        return false;
    };
    sectors != 0 && end <= capacity && end <= LBA48_LIMIT
}
