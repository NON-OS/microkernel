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

use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::State;
use crate::wallet::units::send_decimals;

/// Type into the send amount.
///
/// The point is a key here, not an afterthought. Without it the field could only
/// count in thousandths of an ether, so an exact 0.0005 or a single wei simply
/// could not be said, and the reader had no way to tell that the wallet was
/// refusing rather than that they had mistyped.
pub fn edit_amount(state: &mut State, code: u32) -> Option<EventOutcome> {
    let decimals = send_decimals(state.send_token);
    if code == b'.' as u32 || code == b',' as u32 {
        state.send_amount.start_point();
        return Some(EventOutcome::Repaint);
    }
    let digit = code.checked_sub(b'0' as u32)?;
    if digit < 10 {
        state.send_amount.digit(digit as u8, decimals);
        return Some(EventOutcome::Repaint);
    }
    None
}
