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

//! The lid, as the ACPI capsule last read it from _LID (ACPI 6.5, 9.4.1).
//! Nothing acts on it yet; sleeping when it closes comes with sleep states.

use core::sync::atomic::{AtomicU8, Ordering};

const UNKNOWN: u8 = 0;
const CLOSED: u8 = 1;
const OPEN: u8 = 2;

static LID: AtomicU8 = AtomicU8::new(UNKNOWN);

/// Records the lid; true when that changed what was known.
pub fn record(open: bool) -> bool {
    let now = if open { OPEN } else { CLOSED };
    LID.swap(now, Ordering::AcqRel) != now
}

/// True when open; None before the capsule has said.
pub fn lid_open() -> Option<bool> {
    match LID.load(Ordering::Acquire) {
        OPEN => Some(true),
        CLOSED => Some(false),
        _ => None,
    }
}
