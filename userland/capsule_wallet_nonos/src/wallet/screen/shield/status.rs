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

/*
 * The status line on the Shield screens: the network the pool lives on,
 * whether a shield service answered, and where the keys are. Each part is
 * read from state; none of it is what the wallet would like to be true.
 */

use alloc::vec::Vec;

use crate::wallet::net::route_part;
use crate::wallet::shield::probe::Shield;
use crate::wallet::state::State;

pub fn parts(state: &State) -> Vec<&'static str> {
    let mut out = Vec::new();
    out.push(crate::wallet::chain::current().name);
    out.push(match state.shield {
        Shield::Present => "shield service",
        Shield::Absent => "no shield service",
        Shield::Unknown => "looking",
    });
    /* The network the requests went over, not whether net.nym is up. */
    if let Some(route) = route_part(state.net.route) {
        out.push(route);
    }
    out.push(if state.vault_saved { "sealed" } else { "RAM only" });
    out
}
