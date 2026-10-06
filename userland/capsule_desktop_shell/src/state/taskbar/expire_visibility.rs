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

use super::dock_rule::BRAND_REVEAL_MS;
use super::types::{TaskbarState, Uptime};

/// The once-a-second step of the dock's rule (dock_rule.rs): the dock is
/// shown unless a full-screen window is up, and a brand reveal over such a
/// window lapses after its 1.8 s unless the pointer went to the dock. Returns
/// true when that changed what is drawn.
///
/// It was "the dock is always shown", which a dock found hidden was brought
/// back to; before that it hid the dock 1.8 s after any reveal whenever any
/// window was open, so the dock went away for as long as the wallet's window
/// was up. Only a full-screen window hides it now.
///
/// A reveal is timed on uptime; one set later than `now` reads (a clock
/// that went back) lapses too, rather than stay up until the clock catches
/// up.
pub fn expire_taskbar_visibility(state: &mut TaskbarState, now: Uptime) -> bool {
    let set_at = state.reveal_until_ms.saturating_sub(BRAND_REVEAL_MS);
    if state.reveal_until_ms != 0 && (state.reveal_until_ms <= now.0 || now.0 < set_at) {
        state.reveal_until_ms = 0;
        if !state.pointer_in_dock {
            state.revealed = false;
        }
    }
    super::dock_rule::settle(state)
}
