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

//! A Linux run serving its guest's one listener to one capsule (SERVE_ONE).

use crate::syscall::{call_raw, N_MK_SERVE_ONE};

/// Serve this run's guest listener to the capsule `name`, the run having
/// bound the listing `listing_key` at start. The kernel registers
/// `linux.serve.<name>` for this process, held to that capsule, and returns
/// its port; or a negative errno: EPERM outside the run role or for a pair
/// the kernel does not list, EINVAL for a bad name, EBUSY when the capsule
/// is served or this process serves one, ENOMEM when the registry is full.
pub fn mk_serve_one(name: &[u8], listing_key: &[u8; 32]) -> i64 {
    let key = listing_key.as_ptr() as u64;
    call_raw(N_MK_SERVE_ONE, [name.as_ptr() as u64, name.len() as u64, key, 32, 0, 0])
}
