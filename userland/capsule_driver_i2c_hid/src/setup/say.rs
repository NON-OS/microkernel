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

use crate::state::State;

/// A plain-language console line about the pad.
pub(super) fn say(msg: &str) {
    crate::diag::line(alloc::string::String::from(msg));
}

/// The line that setup is retrying, the first time only: a later line
/// saying why the driver left is never held back by it.
pub(super) fn say_retrying(state: &mut State, msg: &str) {
    if !state.said_retrying {
        state.said_retrying = true;
        say(msg);
    }
}
