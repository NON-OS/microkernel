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

//! One bring-up attempt over every host discovery found.

use crate::emmc::{self, Found, Opened};

/// The hosts in discovery order, Intel eMMC first, and the first whose card
/// comes up is served. The attempt fails, and the bring-up schedule retries
/// it, only when none did; everything a host took is released by then. The
/// reason given is the first host's, the one most likely to be the disk.
pub fn bring_up(hosts: &[Found]) -> Result<Opened, &'static str> {
    let mut first = None;
    for host in hosts.iter() {
        match emmc::open(host) {
            Ok(opened) => return Ok(opened),
            Err(e) => {
                first.get_or_insert(e);
            }
        }
    }
    Err(first.map_or("no host", emmc::reason))
}
