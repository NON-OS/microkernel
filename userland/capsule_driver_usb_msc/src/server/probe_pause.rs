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

//! A probe's pause between readiness tries, spent answering the inbox, and
//! which requests may be answered while a probe is under way.

use nonos_libc::{mk_idle_ms, Deadline};

use super::inbox::Inbox;
use crate::protocol::*;
use crate::scan::{Medium, Report};
use crate::state::State;

/// One pause between readiness tries.
const PROBE_PAUSE_MS: u64 = 50;

impl Inbox {
    /// One pause of a probe, the inbox answered through it.
    pub fn pause(&mut self, state: &mut State, report: &Report) {
        let until = Deadline::after_ms(PROBE_PAUSE_MS);
        while !until.expired() {
            if self.serve(PROBE_PAUSE_MS, state, (&Medium::Scanning, report), true) < 0 {
                let _ = mk_idle_ms(PROBE_PAUSE_MS);
            }
        }
    }
}

/// The block surface answers `E_AGAIN` while scanning, and the state query
/// and health check read nothing a probe holds. The command builders and
/// `OP_PROBE_CONFIG` change bindings and tags a probe is using.
pub(super) fn reads_no_probe_state(op: u16) -> bool {
    matches!(
        op,
        OP_BLK_CAPACITY | OP_BLK_READ | OP_BLK_WRITE | OP_BLK_FLUSH | OP_GET_STATE | OP_HEALTHCHECK
    )
}
