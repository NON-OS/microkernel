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

use super::mark_launch::PULSE_MS;
use super::types::{TaskbarState, Uptime};

/// End each pulse whose time is up at `now`, or that was set later than
/// `now` reads (a clock that went back), rather than hold it until the
/// clock catches up.
pub fn expire_taskbar_pulses(state: &mut TaskbarState, now: Uptime) -> bool {
    let mut dirty = false;
    for pulse in state.pulse_until_ms.iter_mut() {
        let set_at = pulse.saturating_sub(PULSE_MS);
        if *pulse > 0 && (*pulse <= now.0 || now.0 < set_at) {
            *pulse = 0;
            dirty = true;
        }
    }
    dirty
}
