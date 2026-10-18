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

//! Whether a ping must not be sent: only on Direct, and only with an address.

use crate::command::builtin::direct_gate::{refusal_line, PING};
use crate::command::builtin::offline_probe::offline;
use crate::command::output::Output;

// ICMP has no anonymity network to cross, and the name it resolves first
// goes to the resolver in the clear, so a ping is sent only when Direct is
// the chosen network, and only when this machine has an address
// (offline_gate.rs). Checked before the name is resolved.
pub(super) fn refused(out: &mut Output<'_>) -> bool {
    let refused = nonos_route_link::direct_refusal();
    match refusal_line(b"ping", PING, refused).or_else(|| offline(b"ping")) {
        Some(line) => {
            out.writeln(&line);
            true
        }
        None => false,
    }
}
