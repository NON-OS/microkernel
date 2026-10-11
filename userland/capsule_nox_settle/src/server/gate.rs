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

//! settle_proto's caller table (v1.4.7), asked of the registry now: which
//! pid owns each name the op admits.

use nonos_libc::mk_service_lookup;
use nonos_settle_proto::{admits, admits_flags};

fn owner_of(name: &[u8]) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc == 0 && pid != 0).then_some(pid)
}

pub fn admitted(op: u16, sender: u32) -> bool {
    admits(op, sender, owner_of)
}

/// ASYNC (flags bit 0) from payment's or market.index's pid alone (v1.4.23).
pub fn flags_admitted(flags: u16, sender: u32) -> bool {
    admits_flags(flags, sender, owner_of)
}
