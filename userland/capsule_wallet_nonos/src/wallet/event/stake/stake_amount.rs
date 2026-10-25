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

use crate::wallet::nox::NOX_DECIMALS;
use crate::wallet::state::State;

use super::stake_set::set_wei;

// Past thirty figures the amount exceeds any supply that exists.
const MAX_DIGITS: u32 = 30;

/// Type a figure into the stake amount.
///
/// The bar alone cannot express an exact amount: it maps the whole range
/// across a few hundred pixels, so most figures are simply not reachable by
/// dragging. Typing is how a reader says three thousand and means it.
///
/// The figure is held at chain precision with no ceiling of its own. What
/// can actually be staked is decided by the balance at signing time, and
/// silently rewriting a figure as it is entered would hide that refusal.
pub fn digit(state: &mut State, d: u8) -> bool {
    // A figure that arrived from the bar or the max shortcut is a whole
    // amount, not a prefix. Typing over it starts again rather than appending
    // a digit to somebody else's number.
    if state.stake.digits == 0 && state.stake.amount != 0 {
        set_wei(state, 0);
    }
    if state.stake.digits >= MAX_DIGITS {
        return true;
    }
    // Fractions stop at the token's precision: a place the chain cannot
    // count would silently change the amount.
    if state.stake.point && state.stake.places >= NOX_DECIMALS {
        return true;
    }
    state.stake.amount = state.stake.amount.saturating_mul(10).saturating_add(d as u128);
    state.stake.digits += 1;
    if state.stake.point {
        state.stake.places += 1;
    }
    true
}

/// Start the fraction. A second point is ignored rather than refused,
/// because a reader who types one twice meant it once.
pub fn point(state: &mut State) -> bool {
    state.stake.point = true;
    true
}

/// Take back the last keystroke.
pub fn backspace(state: &mut State) -> bool {
    if state.stake.places > 0 {
        state.stake.places -= 1;
    } else if state.stake.point {
        state.stake.point = false;
        return true;
    }
    state.stake.amount /= 10;
    state.stake.digits = state.stake.digits.saturating_sub(1);
    true
}
