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

//! Clearing one name the person typed, as a pure step: every byte written
//! zero, one at a time through a volatile write so it is not left out as a
//! store nothing reads, then the length.
//!
//! No crate paths here, so the host proofs mount this file as it is.

pub(crate) fn clear(name: &mut [u8], len: &mut usize) {
    for b in name.iter_mut() {
        // SAFETY: `b` is a live, exclusive reference into `name`.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    *len = 0;
}
