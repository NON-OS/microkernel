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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, InputKind, PaintBuffer};
use nonos_libc::mk_uptime_ms;

use super::event::on_event;
use super::manifest::manifest;
use super::paint::paint;
use super::state::{hydrate, needs_live_data, new_state, State};

// Time between account refreshes, counted from the end of the last one. One
// batched fetch fills every field, so the wallet does not need to poll often;
// a refresh every dozen seconds keeps balances current.
const REFRESH_MS: i64 = 12_000;

// How long after the last click or keypress a refresh may begin.
const IDLE_MS: i64 = 2_000;

// The tick while a refresh is under way: each tick takes one step of it,
// and no step waits on the network longer than a slice, so a short tick
// moves the refresh along without the window ever waiting for it.
const STEP_TICK_MS: i64 = 30;

// The tick at rest, the skeleton's own.
const REST_TICK_MS: i64 = 1_000;

// The first wait between two reads of a vault or keyring that did not
// answer at boot, doubled each time up to the last.
const HYDRATE_FIRST_MS: i64 = 1_000;
const HYDRATE_MAX_MS: i64 = 15_000;

// How often accounts the list on disk names, and that did not come back at
// boot, are asked for again.
const ACCOUNTS_RETRY_MS: i64 = 5_000;

pub struct Wallet {
    state: State,
    ready: bool,
    /// When the keyring or the vault did not answer yet, the uptime the
    /// next read is due, and how many reads have gone unanswered.
    hydrate_at: i64,
    hydrate_tries: u8,
    last_active: i64,
    last_probe: i64,
    probed_once: bool,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            state: new_state(),
            ready: false,
            hydrate_at: 0,
            hydrate_tries: 0,
            last_active: 0,
            last_probe: 0,
            probed_once: false,
        }
    }
}

impl Wallet {
    /// Whether boot has not settled: the keyring not found yet, or no wallet
    /// and the vault not read to an answer. A vault that answered, opened
    /// or not, is never read again in this window.
    fn unsettled(&self) -> bool {
        let s = &self.state;
        s.keyring_port == 0 || s.owner_pid == 0 || (s.wallet_id == 0 && !s.vault_restore_tried)
    }

    /// Read the keyring and the vault again once the wait is over. True when
    /// it ran.
    fn hydrate_again(&mut self) -> bool {
        let now = mk_uptime_ms();
        if !self.unsettled() || now < self.hydrate_at {
            return false;
        }
        hydrate(&mut self.state);
        let wait = (HYDRATE_FIRST_MS << self.hydrate_tries.min(4)).min(HYDRATE_MAX_MS);
        self.hydrate_tries = self.hydrate_tries.saturating_add(1);
        self.hydrate_at = now + wait;
        true
    }

    /// Whether network work is under way, a refresh or a press's.
    fn stepping(&self) -> bool {
        self.state.net_job.is_some() || self.state.action.is_some()
    }
}

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
        self.state.view_w = fb.width;
        self.state.view_h = fb.height;
        paint(&self.state, fb);
    }

    fn tick_interval_ms(&self) -> i64 {
        if self.stepping() {
            STEP_TICK_MS
        } else {
            REST_TICK_MS
        }
    }

    fn busy(&self) -> bool {
        self.stepping()
    }

    fn on_tick(&mut self) -> bool {
        if !self.ready {
            self.ready = true;
            return self.hydrate_again();
        }
        /* A vault or keyring that did not answer at boot is asked again,
         * waiting longer each time, until it settles. */
        if self.hydrate_again() {
            return true;
        }
        if !self.state.address_ready {
            return false;
        }
        /* Left alone: the window locks by itself (`event::idle_lock`). The
         * clock starts at the first tick with an account, not at boot. */
        let now = mk_uptime_ms();
        if self.last_active == 0 {
            self.last_active = now;
        }
        let sending = self.state.action.is_some();
        if super::event::idle_lock::due(now, self.last_active, self.state.locked, sending) {
            let _ = super::event::lock(&mut self.state);
            self.state.status = b"locked after five minutes without a press";
            return true;
        }
        /* Accounts the list on disk names that did not come back at boot
         * are asked for again every few seconds until they do. */
        if self.state.accounts_owed.is_some() {
            let now = mk_uptime_ms();
            if now >= self.state.accounts_retry_at {
                self.state.accounts_retry_at = now + ACCOUNTS_RETRY_MS;
                super::accounts::resume(&mut self.state);
                return true;
            }
        }
        /* Locked: what is going out finishes, and nothing new begins. */
        if self.state.locked && self.state.action.is_none() {
            return false;
        }
        // A press's network work goes first and alone: the refresh waits
        // while it runs (`act`). A step never waits on the network longer
        // than a slice, and its screen says it is working until it ends.
        if self.state.action.is_some() {
            self.last_probe = mk_uptime_ms();
            return super::act::step(&mut self.state);
        }
        // The shield service's job is asked after every tick while one runs:
        // the call answers at once, so the UI never waits on a proof.
        // A shield service that did not answer in time is asked again here.
        if super::shield::open::retry_due(&mut self.state) {
            return true;
        }
        if self.state.shield_ui.waiting.is_some() || self.state.shield_ui.opened {
            let mut shield = super::shield::job::poll(&mut self.state);
            shield |= super::shield::due(&mut self.state);
            if shield {
                return true;
            }
        }
        // A refresh under way is stepped on every tick, whatever the screen:
        // a step never waits on the network longer than a slice. A new one
        // begins only when the current screen shows live data, no text field
        // is open, and the user has paused, so the Receive screen, where the
        // account is generated, imported, backed up and recovered, starts no
        // network work, while balances and fees still refresh on the screens
        // that display them.
        let live = needs_live_data(self.state.view)
            && !self.state.import_active
            && !self.state.recover_active
            && !self.state.backup_active;
        if self.state.net_job.is_none() && !live {
            return false;
        }
        let now = mk_uptime_ms();
        let idle = now.wrapping_sub(self.last_active) >= IDLE_MS;
        let due = live
            && idle
            && (!self.probed_once
                || self.state.probe_step == 1
                || now.wrapping_sub(self.last_probe) >= REFRESH_MS);
        if due && self.state.net_job.is_none() {
            self.probed_once = true;
        }
        // Repaint only when the refresh changed something on screen, so a
        // steady balance does not recomposite the window.
        let changed =
            matches!(super::event::probe_tick(&mut self.state, due), EventOutcome::Repaint);
        // The interval counts from when the last refresh was still running.
        if self.state.net_job.is_some() || due {
            self.last_probe = now;
        }
        changed
    }
}
