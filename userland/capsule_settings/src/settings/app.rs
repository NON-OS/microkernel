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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};
use nonos_libc::mk_uptime_ms;

use super::event::{on_accessory, on_event};
use super::ipc::hydrate_pass::HYDRATE_RETRY_MS;
use super::ipc::{hydrate, lookup_policy_port};
use super::manifest::manifest;
use super::paint::paint;
use super::section::Section;
use super::state::wifi_enter::refresh_wifi_status;
use super::state::wifi_step::step_wifi;
use super::state::{state_new, State};
use super::ui::search_field;

/// Between the ticks that show and then run a pending Wi-Fi scan or join.
const WIFI_TICK_MS: i64 = 30;
/// Between looks for the answer of a scan or join out on the worker. Not
/// sooner: the worker shares the processor, and its answer takes seconds.
const WIFI_OUT_TICK_MS: i64 = 100;

pub struct Settings {
    state: State,
    hydrated: bool,
    /// When a pass the policy store did not answer is run again, on the
    /// uptime clock.
    hydrate_due_ms: i64,
}

impl Settings {
    pub fn new() -> Self {
        let mut state = state_new();
        if let Ok(port) = lookup_policy_port() {
            state.policy_port = port;
            state.policy_ready = true;
        }
        Settings { state, hydrated: false, hydrate_due_ms: 0 }
    }
}

impl Settings {
    fn ensure_ready(&mut self) {
        if !self.state.policy_ready {
            if let Ok(port) = lookup_policy_port() {
                self.state.policy_port = port;
                self.state.policy_ready = true;
            }
        }
        if self.state.policy_ready && !self.hydrated && self.hydrate_due() {
            self.hydrated = hydrate(&mut self.state);
            self.hydrate_due_ms = mk_uptime_ms().saturating_add(HYDRATE_RETRY_MS);
        }
    }

    fn hydrate_due(&self) -> bool {
        mk_uptime_ms() >= self.hydrate_due_ms
    }
}

impl App for Settings {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        self.ensure_ready();
        on_event(&mut self.state, event)
    }

    fn titlebar_accessory_w(&self) -> u32 {
        search_field::WIDTH
    }

    fn paint_accessory(&mut self, fb: &mut PaintBuffer) {
        search_field::paint(fb, &self.state);
    }

    fn on_accessory_event(&mut self, event: InputEvent) -> EventOutcome {
        on_accessory(&mut self.state, event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.ensure_ready();
        // The window resizes, so record what is actually being painted into.
        // Hit tests and row counts read this back.
        self.state.win_w = fb.width;
        self.state.win_h = fb.height;
        paint(&self.state, fb);
    }

    // While the Wi-Fi or Network page is open, re-poll net_core every tick so a
    // lease that binds a few seconds after connecting shows its address without
    // the user having to trigger another scan.
    // A scan or join asked for is shown and run on the next two ticks, so
    // those come quickly rather than a second apart.
    // One handed to the worker is looked for every 100 ms until it answers.
    fn tick_interval_ms(&self) -> i64 {
        let pending = self.state.wifi.pending;
        if pending.is_asked() {
            WIFI_TICK_MS
        } else if pending.is_out() {
            WIFI_OUT_TICK_MS
        } else {
            1000
        }
    }

    // Only a request still to be shown and handed over wants the runner to
    // yield rather than sleep: one that is out is the worker's to advance.
    fn busy(&self) -> bool {
        self.state.wifi.pending.is_asked()
    }

    fn on_tick(&mut self) -> bool {
        // A pass the policy store did not answer runs again from here, so the
        // stored values replace the defaults without a key or a click.
        if self.state.policy_ready && !self.hydrated && self.hydrate_due() {
            self.ensure_ready();
            return true;
        }
        // A scan or join asked for runs from here, a tick after the panel
        // said it would, whatever page is open by then.
        if step_wifi(&mut self.state) {
            return true;
        }
        if matches!(self.state.section, Section::Wifi | Section::Network) {
            refresh_wifi_status(&mut self.state);
            return true;
        }
        false
    }
}
