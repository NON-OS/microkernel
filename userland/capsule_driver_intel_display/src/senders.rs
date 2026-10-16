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

//! Who may set brightness and a monitor's controls, and read them: the
//! compositor alone. The kernel lets no other process reach a display
//! driver (its held table), so a right named for anyone else could never be
//! used; the compositor takes these requests from the desktop shell alone
//! and relays them. Looked up by name on every request, so a restarted
//! compositor is known by its new pid and a stale pid keeps no right.

use nonos_libc::mk_service_lookup;

/// The registry names allowed to set brightness.
const SETTERS: [&str; 1] = ["compositor"];

/// And those allowed to read a monitor's own controls (VCP).
const READERS: [&str; 1] = ["compositor"];

pub fn may_set(sender: u32) -> bool {
    named(sender, &SETTERS)
}

pub fn may_read_controls(sender: u32) -> bool {
    named(sender, &READERS)
}

fn named(sender: u32, names: &[&str]) -> bool {
    sender != 0 && names.iter().any(|n| pid_of(n) == Some(sender))
}

fn pid_of(name: &str) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc >= 0 && pid != 0).then_some(pid)
}
