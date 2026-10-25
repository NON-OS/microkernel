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

//! BERT, the Boot Error Record Table (ACPI 6.5, 18.3.1): where the boot
//! error region is, holding the fatal error that ended the last boot.

use super::le::u32_at;

/// The region's address and length; none when firmware left it empty.
pub fn bert(t: &[u8]) -> Option<(u64, u32)> {
    let len = t.get(36..40).map(|_| u32_at(t, 36))?;
    let addr = u64::from_le_bytes(t.get(40..48)?.try_into().ok()?);
    (addr != 0 && len != 0).then_some((addr, len))
}
