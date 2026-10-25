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

//! Whether `nslookup` may go out now: not where direct routes are refused, nor offline.

use crate::command::builtin::direct_gate::{refusal_line, NSLOOKUP};
use crate::command::builtin::offline_probe::offline;
use crate::term::state::State;

/// False, having said why, when the lookup must not go out.
pub(super) fn may_ask(state: &mut State) -> bool {
    let refused = nonos_route_link::direct_refusal();
    if let Some(line) = refusal_line(b"nslookup", NSLOOKUP, refused) {
        state.scrollback.push_error(&line);
        return false;
    }
    if let Some(line) = offline(b"nslookup") {
        state.scrollback.push_error(&line);
        return false;
    }
    true
}
