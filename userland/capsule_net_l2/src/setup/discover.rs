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

use nonos_libc::mk_service_lookup;

// The kernel's table of the cards it starts; net.l2 is held to the wired
// ones only.
#[path = "../../../../src/services/registry/nics.rs"]
mod nics;

use nics::{Link, NICS};

#[derive(Clone, Copy)]
pub struct Nic {
    pub port: u32,
    pub pid: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DiscoverError {
    NotFound,
}

pub fn first_available() -> Result<Nic, DiscoverError> {
    // The wired cards in the table's order: the first one the kernel
    // registry knows wins, and the data plane upstream of L2 sees no
    // difference between them.
    for (name, _) in NICS.iter().filter(|(_, link)| *link == Link::Wired) {
        let mut port: u32 = 0;
        let mut pid: u32 = 0;
        let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
        if rc == 0 {
            return Ok(Nic { port, pid });
        }
    }
    Err(DiscoverError::NotFound)
}
