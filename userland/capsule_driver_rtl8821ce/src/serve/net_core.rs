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

//! Which process owns net.core now, as the kernel's registry says. Asked on
//! each link request rather than kept, so a net.core that restarts under a
//! new pid is followed, and none at all admits nobody.

use nonos_libc::mk_service_lookup;

const NET_CORE: &[u8] = b"net.core";

pub(super) fn net_core_pid() -> Option<u32> {
    let mut pid = 0u32;
    let found =
        mk_service_lookup(NET_CORE.as_ptr(), NET_CORE.len(), core::ptr::null_mut(), &mut pid);
    (found >= 0 && pid != 0).then_some(pid)
}
