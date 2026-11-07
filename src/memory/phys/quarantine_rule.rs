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

//! The quarantine's list, pure: kernel_proofs::surface_import.

extern crate alloc;

use alloc::vec::Vec;

/// Add `[start, start + len)`; an empty run adds nothing.
pub(crate) fn add(runs: &mut Vec<(u64, u64)>, start: u64, len: u64) {
    if len != 0 && !runs.contains(&(start, len)) {
        runs.push((start, len));
    }
}

/// Whether `[start, start + len)` shares a byte with a held run.
pub(crate) fn meets(runs: &[(u64, u64)], start: u64, len: u64) -> bool {
    let end = start.saturating_add(len);
    runs.iter().any(|&(s, l)| start < s.saturating_add(l) && s < end)
}
