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

//! What the driver keeps for one host: the hubs found through it and its
//! root ports' tries; and the lookup of a host by name.

use alloc::vec::Vec;

use nonos_usb_hub::Hub;

pub struct Held {
    pub hubs: Vec<Hub>,
    pub tried: [u8; 256],
}

impl Held {
    pub fn new() -> Self {
        Self { hubs: Vec::new(), tried: [0; 256] }
    }
}

pub fn lookup(name: &str) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = nonos_libc::mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc == 0).then_some(port)
}
