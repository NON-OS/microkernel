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

//! A tick in order: a consent's focus, then the press, shield and meter
//! jobs it steps.

use super::{mk_uptime_ms, Wallet};

impl Wallet {
    /// One tick: the first checks, then the jobs, then the refresh.
    pub(super) fn tick(&mut self) -> bool {
        // A consent's focus first, and a guarded screen's (order/guard, arm).
        let mut guard = crate::wallet::order::guard::poll::poll(&mut self.state);
        guard |= crate::wallet::arm::tick::poll(&mut self.state);
        if let Some(r) = self.tick_gate() {
            return r || guard;
        }
        if let Some(r) = self.tick_jobs() {
            return r || guard;
        }
        self.tick_refresh() || guard
    }

    /// A press's, the shield's or the meter's job. Some when one ran.
    fn tick_jobs(&mut self) -> Option<bool> {
        // A press's network work goes first and alone: the refresh waits
        // while it runs (`act`). A step never waits on the network longer
        // than a slice, and its screen says it is working until it ends.
        if self.state.action.is_some() {
            self.last_probe = mk_uptime_ms();
            return Some(super::act::step(&mut self.state));
        }
        // The shield service's job is asked after every tick while one runs:
        // the call answers at once, so the UI never waits on a proof.
        // A shield service that did not answer in time is asked again here.
        if super::shield::open::retry_due(&mut self.state) {
            return Some(true);
        }
        // The meter's job, likewise asked after every tick while one runs.
        if super::meter::poll::tick(&mut self.state) {
            return Some(true);
        }
        if self.state.shield_ui.waiting.is_some() || self.state.shield_ui.opened {
            let mut shield = super::shield::job::poll(&mut self.state);
            shield |= super::shield::due(&mut self.state);
            if shield {
                return Some(true);
            }
        }
        None
    }
}
