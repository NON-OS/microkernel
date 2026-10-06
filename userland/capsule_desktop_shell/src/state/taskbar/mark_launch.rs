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

use super::types::{TaskbarState, Uptime};

/// How long a launched app's dock icon pulses.
pub const PULSE_MS: i64 = 900;

pub fn mark_taskbar_launch(state: &mut TaskbarState, index: usize, now: Uptime) {
    if index < state.pulse_until_ms.len() {
        state.pulse_until_ms[index] = now.0.saturating_add(PULSE_MS).max(1);
    }
}
