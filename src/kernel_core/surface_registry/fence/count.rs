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

//! The counts. One per slot; it is never reset, so a count read once is
//! only ever passed, never met again by wrapping within any wait.

use core::sync::atomic::{AtomicU32, Ordering};

use super::super::types::SLOT_CAP;

static COUNTS: [AtomicU32; SLOT_CAP] = [const { AtomicU32::new(0) }; SLOT_CAP];

pub(super) fn count(idx: usize) -> Option<u32> {
    COUNTS.get(idx).map(|c| c.load(Ordering::Acquire))
}

pub(super) fn bump(idx: usize) -> Option<u32> {
    COUNTS.get(idx).map(|c| c.fetch_add(1, Ordering::AcqRel).wrapping_add(1))
}
