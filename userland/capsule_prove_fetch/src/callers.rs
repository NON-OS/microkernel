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

//! Who is asking, by the kernel's service registry: the pid that owns
//! app.store, or market.index. A lookup that misses names no one.

use nonos_libc::mk_service_lookup;

use crate::desk::Caller;

fn owner(service: &[u8]) -> Option<u32> {
    let mut pid = 0u32;
    let rc = mk_service_lookup(service.as_ptr(), service.len(), core::ptr::null_mut(), &mut pid);
    (rc == 0 && pid != 0).then_some(pid)
}

pub fn caller(pid: u32) -> Caller {
    Caller {
        pid,
        store: owner(b"app.store") == Some(pid),
        index: owner(b"market.index") == Some(pid),
    }
}
