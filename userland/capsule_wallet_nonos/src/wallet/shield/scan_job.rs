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

//! The restore scan's batches from the Shield (B7, v1.4.56): a job deriving
//! the next addresses past the accounts, asked only once the person pressed
//! scan or scan further. A job that ends unanswered stops the scan.

use alloc::format;

use shield_wire::{fields, FRESH_BATCH, OP_FRESH_ADDRESS};

use super::landed::parse_handed;
use super::scan::ScanState;
use crate::wallet::state::State;

/// Ask the Shield for the batch the scan waits on. Whether the screen changed.
pub fn derive(state: &mut State) -> bool {
    let ScanState::Deriving(from) = state.meter.landed.scan.state else { return false };
    let waiting = state.shield_ui.waiting.is_some();
    if state.meter.landed.derive_asked {
        if !waiting {
            state.meter.landed.derive_asked = false;
            state.meter.landed.scan.state = ScanState::Stopped;
            return true;
        }
        return false;
    }
    if waiting {
        return false;
    }
    let from = format!("{from}");
    let started = super::job::start(state, OP_FRESH_ADDRESS, &[FRESH_BATCH, &from]);
    state.meter.landed.derive_asked = started;
    started
}

/// The job's answer: the batch the scan reads next.
pub fn derived(state: &mut State, body: &str) {
    let l = &mut state.meter.landed;
    l.derive_asked = false;
    if matches!(l.scan.state, ScanState::Deriving(_)) {
        l.scan.batch(parse_handed(fields(body, "handed")));
    }
}

/// The scan button: only here does a restore scan begin.
pub fn start(state: &mut State) {
    state.meter.landed.scan.start();
}

pub fn further(state: &mut State) {
    state.meter.landed.scan.further();
}
