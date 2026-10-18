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

//! Zeroing what the bus carried once it is handed on: a request once it is
//! answered, a reply once it is sent, and the engine's copy of the bytes a
//! transfer read. Nothing a client sent or a device returned stays in this
//! driver after it is delivered.

use core::sync::atomic::{compiler_fence, Ordering};

/// Zero `bytes` with writes the compiler may not drop as dead, though
/// nothing reads them afterwards.
pub fn wipe(bytes: &mut [u8]) {
    for b in bytes.iter_mut() {
        // SAFETY: `b` is a live, exclusive, aligned reference to one byte.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    compiler_fence(Ordering::SeqCst);
}
