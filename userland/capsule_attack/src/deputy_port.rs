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

//! The desktop shell's port, for the deputy case (deputy.rs).

use nonos_libc::{mk_service_lookup, mk_time_millis, mk_yield};

/// The shell's port, waited for: init starts it before this capsule, but it
/// names itself on its own first turn.
pub fn shell_port() -> Option<u32> {
    let (name, until) = (b"desktop_shell", mk_time_millis() + 20_000);
    let mut port = 0u32;
    while mk_service_lookup(name.as_ptr(), name.len(), &mut port, core::ptr::null_mut()) < 0 {
        if mk_time_millis() > until {
            return None;
        }
        mk_yield();
    }
    Some(port)
}
