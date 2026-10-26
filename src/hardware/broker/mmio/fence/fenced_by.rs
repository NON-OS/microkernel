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

//! Read by the ACPI broker before it admits a late device's windows.

use super::{overlaps, FENCED};

/// The holder whose fence a window falls in, if any.
pub(in crate::hardware::broker) fn fenced_by(start: u64, len: u64) -> Option<u32> {
    FENCED.lock().iter().find(|f| overlaps(f.start, f.len, start, len)).map(|f| f.pid)
}
