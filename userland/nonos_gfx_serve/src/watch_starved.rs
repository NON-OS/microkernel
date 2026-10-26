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
//! A starved owner is not a dead one (lead, 26 October). While capsules start
//! on one processor, the compositor can wait seconds for it, or for peers it
//! calls that wait too, and goes unheard. A live one's frame then stays:
//! blanking it only makes the screen flash. One gone, stopped or ended is
//! blanked at OWNER_SILENCE_MS as before; one alive and still unheard by
//! OWNER_HUNG_MS is taken as hung, and is blanked then. A row younger than
//! the silence is another process on a reused pid: the owner was alive when
//! last heard, so that row is not it, and it counts as gone.

use super::display::Display;
use super::platform::Platform;
use super::server::Server;

/// How long a live owner's frame may stay unheard.
pub const OWNER_HUNG_MS: u64 = 10_000;

/// A process-table row (MkProcStat) of the owner unheard for `quiet_ms`
/// that lives: state 1 ready, 2 running or 3 asleep, and alive for longer
/// than the silence, so the same process that was last heard.
pub fn lives(state: u8, alive_ms: u64, quiet_ms: u64) -> bool {
    matches!(state, 1..=3) && alive_ms > quiet_ms
}

impl<D: Display, P: Platform> Server<D, P> {
    /// True while an owner unheard for `quiet_ms` still lives, and is not
    /// yet so long unheard that it is taken as hung.
    pub(super) fn starved(&self, quiet_ms: u64) -> bool {
        quiet_ms < OWNER_HUNG_MS && self.owner.is_some_and(|o| self.platform.lives(o, quiet_ms))
    }
}
