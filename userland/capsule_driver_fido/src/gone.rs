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
//! A key gone from its port, root or virtual: its holder let go, its
//! slot given back and its port free for the search again.

use nonos_usbnet::say::say;
use nonos_usbnet::xhci::{connected_ports, disable_slot};

use crate::device::Dev;
use crate::search::Search;

pub fn gone(d: &Dev) -> bool {
    let Some(b) = d.bound.as_ref() else { return false };
    connected_ports(b.xhci).is_ok_and(|ps| !ps.iter().any(|p| p.id == b.port))
}

/// Its request went with it; a key plugged in again gets a new channel.
pub fn give_back(d: &mut Dev, search: &mut Search) {
    d.owner.close();
    d.job = crate::ctaphid::Job::Idle;
    d.spent = false;
    if let Some(b) = d.bound.take() {
        say(b"fido", &[b"key unplugged, given back"]);
        // Shared, the host keeps the slot for the other driver.
        disable_slot(b.xhci, b.slot);
        search.lost(&b);
    }
}
