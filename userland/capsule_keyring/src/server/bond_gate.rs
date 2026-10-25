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

//! The bond gate, asked of the registry (L33).

mod rule;

use nonos_libc::mk_service_lookup;

use crate::store::Store;

/// The holder name `sender_pid` serves now, by the registry's word.
pub fn holder(sender_pid: u32) -> Option<u8> {
    rule::holder_of(sender_pid, |name| {
        let (mut port, mut pid) = (0u32, 0u32);
        let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
        (rc == 0 && pid != 0).then_some(pid)
    })
}

/// Before any request of `sender_pid`'s is answered, hand it the records
/// kept for the name it serves: a restarted driver finds its bonds.
pub fn adopt(store: &mut Store, sender_pid: u32) {
    if !store.awaits_holder(sender_pid) {
        return;
    }
    if let Some(at) = holder(sender_pid) {
        store.adopt(at, sender_pid);
    }
}
