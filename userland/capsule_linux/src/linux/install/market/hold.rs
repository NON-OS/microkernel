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

//! Whether this install is of a market release, whose package's programs are
//! held (minted.rs) until the release's policy is bound to them, and the
//! bytes that policy names, the program the install records (choose.rs). The
//! install role installs one package a process, so both are set once, before
//! the package is fetched.

use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

static HELD: AtomicBool = AtomicBool::new(false);
static PROGRAM: [AtomicU8; 32] = [const { AtomicU8::new(0) }; 32];

pub(in crate::linux::install) fn hold(program: &[u8; 32]) {
    for (slot, b) in PROGRAM.iter().zip(program) {
        slot.store(*b, Ordering::SeqCst);
    }
    HELD.store(true, Ordering::SeqCst);
}

/// The BLAKE3 of the program a held install's policy names.
pub(in crate::linux::install) fn wanted() -> Option<[u8; 32]> {
    held().then(|| core::array::from_fn(|i| PROGRAM[i].load(Ordering::SeqCst)))
}

pub(in crate::linux::install) fn held() -> bool {
    HELD.load(Ordering::SeqCst)
}
