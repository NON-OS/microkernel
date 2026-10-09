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

//! How far the Anyone network has got, as a SOCKS caller is told it.
//!
//! A browser on a cold boot met "not connected yet" at once and showed it as
//! the page's failure, while the bootstrap went on for minutes behind it.
//! The caller can now ask how far it is and say so while it waits.

use crate::circuit::CircuitStage;
use crate::manager::{Bootstrap, Manager};

use super::super::handlers::not_ready;
use super::tunnel::Progress;

pub fn progress(state: &Manager, now: u64) -> Progress {
    let stage = match state.bootstrap {
        Bootstrap::Cold => 0,
        Bootstrap::Anchored => 1,
        Bootstrap::Joining => 2,
        Bootstrap::Ready => 3,
    };
    let open = state.circuits.iter().filter(|c| c.stage == CircuitStage::Open).count();
    let opens = not_ready(state, now).is_none();
    Progress::of(stage, state.refreshing, opens, state.link.is_some(), open)
}
