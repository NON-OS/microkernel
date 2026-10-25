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

//! The skeleton's `App` hooks for the wallet window.

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, InputKind, PaintBuffer};

use super::{manifest, mk_uptime_ms, on_event, paint, Wallet, REST_TICK_MS, STEP_TICK_MS};

impl App for Wallet {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        if !self.ready {
            self.ready = true;
            self.hydrate_again();
        }
        // Only a real click or keypress holds a new refresh back, so one never
        // begins under the hand. Mouse movement must NOT count, or moving the
        // cursor would starve the refresh and later reads (fee, staking) would
        // never complete.
        if matches!(event.kind, InputKind::ButtonDown | InputKind::KeyDown) {
            self.last_active = mk_uptime_ms();
        }
        on_event(&mut self.state, event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        // Record the width the screens lay out against so pointer handlers can
        // hit-test the same rectangles for width-relative controls.
        self.state.frame.w = fb.width;
        self.state.frame.h = fb.height;
        paint(&self.state, fb);
        crate::wallet::order::guard::poll::drawn(&mut self.state);
        crate::wallet::arm::gate::drawn(&mut self.state);
    }

    fn tick_interval_ms(&self) -> i64 {
        if self.stepping() || crate::wallet::arm::gate::looking(&self.state) {
            STEP_TICK_MS
        } else {
            REST_TICK_MS
        }
    }

    fn busy(&self) -> bool {
        self.stepping()
    }

    /* settle's ASYNC answer for a dispute or refund (v1.4.53); any other
     * frame not from the router is dropped, as the skeleton always did. */
    fn peer_frame(&mut self, sender: u32, frame: &[u8]) -> bool {
        self.state.meter.settle.frame(sender, frame)
    }

    fn on_tick(&mut self) -> bool {
        self.tick()
    }
}
