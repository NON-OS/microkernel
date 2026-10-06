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

use core::sync::atomic::{AtomicU32, Ordering};

/*
 * The pool's service port is fixed for the life of the system, and it was
 * looked up with a syscall before every signature check. Kept after the first
 * lookup that succeeds; a failed lookup is not kept, so a pool that was not
 * up yet is found later. Any CPU may race the first store, and every racer
 * stores the same port.
 */
static PORT: AtomicU32 = AtomicU32::new(0);

pub fn crypto_port() -> Option<u32> {
    let known = PORT.load(Ordering::Acquire);
    if known != 0 {
        return Some(known);
    }
    let mut port = 0u32;
    let mut pid = 0u32;
    let name = b"crypto_pool";
    let rc = nonos_libc::mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 || port == 0 {
        None
    } else {
        PORT.store(port, Ordering::Release);
        Some(port)
    }
}
