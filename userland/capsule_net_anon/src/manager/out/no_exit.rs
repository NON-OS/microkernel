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

//! Whether a stream's port is one no circuit will ever carry under the
//! directory in hand. Only a whole directory says so: relays keep joining
//! as their microdescriptors land after the first path is drawn, and a
//! refreshed consensus may bring exits the one serving lacks. Until every
//! batch is asked and no refresh is under way, a port no exit in hand lets
//! out waits as any other, as it always did.

use crate::path::{any_exit, Relay};

use super::super::batch::batch_count;
use super::super::state::Manager;

pub(super) fn no_exit(state: &Manager, port: u16) -> bool {
    let takes = |exit: &Relay| state.exits.takes(exit, port);
    whole(state) && !any_exit(&state.relays, &state.weights, takes)
}

fn whole(state: &Manager) -> bool {
    !state.refreshing && state.dir.micro >= batch_count(&state.entries)
}
