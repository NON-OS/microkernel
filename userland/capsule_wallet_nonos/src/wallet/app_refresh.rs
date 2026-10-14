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

//! The account refresh a tick begins or steps.

use nonos_app_skeleton::EventOutcome;

use super::{mk_uptime_ms, needs_live_data, Wallet};

// Time between account refreshes, counted from the end of the last one. One
// batched fetch fills every field, so the wallet does not need to poll often;
// a refresh every dozen seconds keeps balances current.
const REFRESH_MS: i64 = 12_000;

// How long after the last click or keypress a refresh may begin.
const IDLE_MS: i64 = 2_000;

impl Wallet {
    /// Step a refresh under way, or begin one when it is due.
    pub(super) fn tick_refresh(&mut self) -> bool {
        // A refresh under way is stepped on every tick, whatever the screen:
        // a step never waits on the network longer than a slice. A new one
        // begins only when the current screen shows live data, no text field
        // is open, and the user has paused, so the Receive screen, where the
        // account is generated, imported, backed up and recovered, starts no
        // network work, while balances and fees still refresh on the screens
        // that display them.
        let live = needs_live_data(self.state.view)
            && !self.state.keys.import_active
            && !self.state.keys.recover_active
            && !self.state.keys.backup_active;
        if self.state.net.job.is_none() && !live {
            return false;
        }
        let now = mk_uptime_ms();
        let idle = now.wrapping_sub(self.last_active) >= IDLE_MS;
        let due = live
            && idle
            && (!self.probed_once
                || self.state.net.probe_step == 1
                || now.wrapping_sub(self.last_probe) >= REFRESH_MS);
        if due && self.state.net.job.is_none() {
            self.probed_once = true;
        }
        // Repaint only when the refresh changed something on screen, so a
        // steady balance does not recomposite the window.
        let changed =
            matches!(super::event::probe_tick(&mut self.state, due), EventOutcome::Repaint);
        // The interval counts from when the last refresh was still running.
        if self.state.net.job.is_some() || due {
            self.last_probe = now;
        }
        changed
    }
}
