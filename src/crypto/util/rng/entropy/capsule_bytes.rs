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

//! The one accumulator the entropy capsule's replies feed and every seed
//! draws from. Each takes its lock for one batch; no IPC, sleep or
//! wait happens under it.

use spin::Mutex;

use super::accumulator::{Accumulator, Fed, HELD};

static CAPSULE_BYTES: Mutex<Accumulator> = Mutex::new(Accumulator::new());

/// One reply's bytes, through the health tests.
pub fn feed(batch: &[u8]) {
    let fed = CAPSULE_BYTES.lock().feed(batch);
    if let Fed::Refused(fault) = fed {
        crate::log_warn!("entropy: capsule bytes refused, {:?} alarm", fault);
    }
}

/// What is held, and how many bytes of it count, for a seed.
pub(super) fn take() -> Option<([u8; HELD], usize)> {
    CAPSULE_BYTES.lock().take()
}

/// Whether more bytes are wanted: none held yet, or the source unproven.
pub fn wanted() -> bool {
    CAPSULE_BYTES.lock().wanted()
}
