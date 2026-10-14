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

//! The wallet window: its state, the tick rates, and the parts of the app.

use nonos_libc::mk_uptime_ms;

use super::event::on_event;
use super::manifest::manifest;
use super::paint::paint;
use super::state::{hydrate, needs_live_data, new_state, State};
use super::{accounts, act, event, meter, shield};

#[path = "app_hydrate.rs"]
mod boot;
#[path = "app_gate.rs"]
mod gate;
#[path = "app_impl.rs"]
mod imp;
#[path = "app_jobs.rs"]
mod jobs;
#[path = "app_refresh.rs"]
mod refresh;

// The tick while a refresh is under way: each tick takes one step of it,
// and no step waits on the network longer than a slice, so a short tick
// moves the refresh along without the window ever waiting for it.
const STEP_TICK_MS: i64 = 30;

// The tick at rest, the skeleton's own.
const REST_TICK_MS: i64 = 1_000;

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
