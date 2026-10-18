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

//! Finding the driver the server plays through.

use nonos_libc::{mk_idle_ms, mk_service_lookup};

/// The kernel registers a sink driver's name when it spawns the driver,
/// before this capsule, and drops it when that driver exits (no controller,
/// or one that never came up). The lookup is retried for about two seconds,
/// asleep between tries; it used to be 2000 tries a yield apart, which on an
/// idle machine is a spin and on a busy one has no fixed length.
const LOOKUP_RETRIES: u32 = 100;
const LOOKUP_PAUSE_MS: u64 = 20;

/// What `find` looks up, once it is registered.
pub(super) fn resolve<T>(mut find: impl FnMut() -> Option<T>) -> Option<T> {
    (0..LOOKUP_RETRIES).find_map(|tries| {
        if tries > 0 {
            let _ = mk_idle_ms(LOOKUP_PAUSE_MS);
        }
        find()
    })
}

/// The port `name` is served on, if it is registered now.
pub(super) fn once(name: &[u8]) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let r = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (r == 0 && port != 0).then_some(port)
}
