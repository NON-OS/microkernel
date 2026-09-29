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

//! Whether the foreground program reads raw keys, and handing it bytes.

use nonos_vt::{MouseMode, Term};

use crate::jobs::{JobWork, EOT};
use crate::term::state::State;

pub fn reads_raw(vt: &Term) -> bool {
    let m = &vt.modes;
    vt.alt_active() || m.cursor_keys || m.bracketed_paste || m.mouse != MouseMode::Off
}

/// Queue bytes for the foreground program. False when it cannot take input.
pub fn send(state: &mut State, bytes: &[u8]) -> bool {
    let Some(id) = state.jobs.foreground() else { return false };
    match state.jobs.get_mut(id).map(|j| &mut j.work) {
        Some(JobWork::ExternalStage { stdin, .. }) => stdin.push(bytes),
        _ => false,
    }
}

/// Ctrl+D on an empty line: end the foreground program's input, when it is
/// one that hears a lone 0x04 as the end. False when it is not.
pub(super) fn end_input(state: &mut State) -> bool {
    let Some(id) = state.jobs.foreground() else { return false };
    match state.jobs.get_mut(id).map(|j| &mut j.work) {
        Some(JobWork::ExternalStage { stdin, .. }) if stdin.hears_eot => stdin.push(&[EOT]),
        _ => false,
    }
}

pub(super) fn takes_input(state: &State) -> bool {
    let Some(id) = state.jobs.foreground() else { return false };
    matches!(state.jobs.get(id).map(|j| &j.work), Some(JobWork::ExternalStage { .. }))
}
