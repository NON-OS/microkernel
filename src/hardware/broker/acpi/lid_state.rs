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

//! The lid, as the ACPI capsule last read it from _LID (ACPI 6.5, 9.4.1),
//! and told to the shell (F-91) for its lock. Nothing in the kernel acts on
//! it: whether a close powers off is the owner's open row.

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Report {
    /// The same as known: nothing to tell.
    Unchanged,
    Told,
    /// The shell could not take it; nothing recorded, so the capsule's next
    /// reading tells it again.
    Busy,
}

/// A reading, told to the shell only when it changes what is known, and
/// recorded only once told.
pub fn report(open: bool, tell: impl FnOnce(bool) -> bool) -> Report {
    let (r, now) = decide(LID.load(Ordering::Acquire), open, tell);
    LID.store(now, Ordering::Release);
    r
}

/// The rule, pure: what to answer and what is then known.
pub fn decide(known: u8, open: bool, tell: impl FnOnce(bool) -> bool) -> (Report, u8) {
    let now = if open { OPEN } else { CLOSED };
    match (known == now, known == now || tell(open)) {
        (true, _) => (Report::Unchanged, known),
        (false, true) => (Report::Told, now),
        (false, false) => (Report::Busy, known),
    }
}
