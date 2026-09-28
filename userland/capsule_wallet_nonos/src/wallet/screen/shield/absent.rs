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

/*
 * What a Shield screen says when there is no shield service to do the work.
 * The screens stay usable for reading and choosing; only the step that would
 * hand work to the service is held back, and this is the sentence saying why.
 */

use crate::wallet::shield::probe::Shield;
use crate::wallet::state::State;

const ABSENT: &str = "No shield service is running on this machine. You can look and \
     choose, but nothing is deposited, sent or withdrawn until one is installed.";
const LOOKING: &str = "Looking for the shield service.";

pub fn ready(state: &State) -> bool {
    state.shield.available()
}

/* The banner for this screen: a failure the service reported, else absence. */
pub fn banner(state: &State) -> Option<&'static str> {
    if let Some(f) = state.shield_ui.failure {
        return Some(f);
    }
    match state.shield {
        Shield::Present => None,
        Shield::Absent => Some(ABSENT),
        Shield::Unknown => Some(LOOKING),
    }
}
