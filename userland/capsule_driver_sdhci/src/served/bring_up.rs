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

use crate::emmc::{self, EmmcError, Found, Opened};

/// The internal disk first: the hosts in discovery order, Intel eMMC
/// first, and the first whose card comes up is served. Only when every
/// host was refused for not being embedded, so this machine has no eMMC
/// host at all, is the first removable slot served, card or no card. The
/// attempt fails, and the bring-up schedule retries it, when nothing was
/// served; everything a host took is released by then. The reason given
/// is the first host's, the one most likely to be the disk.
pub fn bring_up(hosts: &[Found]) -> Result<Opened, &'static str> {
    let mut first = None;
    let mut only_readers = !hosts.is_empty();
    for host in hosts.iter() {
        match emmc::open(host) {
            Ok(opened) => {
                if let Some(d) = &opened.slot.disk {
                    d.say_part();
                }
                return Ok(opened);
            }
            Err(e) => {
                only_readers &= e == EmmcError::NotEmbedded;
                first.get_or_insert(e);
            }
        }
    }
    if only_readers {
        if let Some(opened) = hosts.iter().find_map(|h| emmc::open_removable(h).ok()) {
            return Ok(opened);
        }
    }
    Err(first.map_or("no host", emmc::reason))
}
