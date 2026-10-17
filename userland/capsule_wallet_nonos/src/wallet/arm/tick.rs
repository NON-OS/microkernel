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

//! The guard's look at focus each tick (gate.rs): kept on `State.arm`, and
//! the frame painted again whenever its guarded presses arm or disarm.

use super::gate::own;
use super::look::seen;
use super::shows::relit;
use crate::wallet::order::guard::poll::{focused, now};
use crate::wallet::state::State;

/// One look at focus while a guarded screen shows; whether the guarded
/// presses' look flipped, so the frame must be painted again.
pub fn poll(state: &mut State) -> bool {
    let (shown, now) = (own(state), now());
    if shown.is_some() || state.arm.shown.is_some() {
        let focused = shown.is_some() && focused();
        state.arm = seen(state.arm, shown, focused, now);
    }
    let (lit, dirty) = relit(state.arm_lit, &state.arm, shown, now);
    state.arm_lit = lit;
    dirty
}
