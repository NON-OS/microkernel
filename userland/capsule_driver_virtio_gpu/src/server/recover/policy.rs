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

//! What the server does after every request: a device that has served for
//! SERVED_MS since its last recovery has its recoveries in a row forgotten,
//! and a stalled queue is recovered, once per stall, until they run out.

use super::tries::{down, recover, IN_A_ROW_DOWN};
use crate::clock::now_ms;
use crate::constants::CTRLQ_MS;
use crate::driver::Driver;
use crate::state::Health;

/// Recoveries in a row after which the next stall leaves the driver `Down`:
/// three, as many as the tries one stall gets.
pub const IN_A_ROW: u32 = 3;

/// How long a recovered device must serve, by the clock, before its
/// recoveries stop counting: in service throughout, a client command
/// answered. The count used to clear at any answer, so a device that
/// answered one present per recovery and stalled the next was recovered
/// forever, a stall every 2 s or so, reporting Live. Now one that stalls
/// again within SERVED_MS of each recovery is `Down` at its fourth stall:
/// some 8 s of stalled presents before the compositor is told by name.
/// Fifteen CTRLQ_MS, long against one stall, so a device that keeps
/// stalling is frozen at most CTRLQ_MS in every SERVED_MS or so; short
/// against a session, so a host that stalled for a while and serves again
/// is forgiven within half a minute.
pub const SERVED_MS: u64 = 15 * CTRLQ_MS;

/// After every request. A driver already down is left answering E_DEVICE.
pub fn settle(driver: &mut Driver) {
    let q = &driver.control_queue;
    if !q.is_reset() {
        // In service, so no command stalled since the last recovery ended.
        let answered = q.published() != driver.recovered_at;
        if answered && now_ms().saturating_sub(driver.recovered_ms) >= SERVED_MS {
            driver.in_a_row = 0;
        }
    } else if driver.health == Health::Live {
        // The reset took render context 1 with every host resource.
        driver.virgl_ready = false;
        if driver.in_a_row < IN_A_ROW {
            recover(driver);
        } else {
            down(driver, IN_A_ROW_DOWN);
        }
    }
}
