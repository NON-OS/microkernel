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
 * The guard on a screen whose presses move value or sign, beside the
 * purchase's own (order/guard): armed as Buy is, by the one arming of
 * order/terms/arming.rs. Off the purchase there are no terms to read
 * again, so focus lost disarms it, and only a fresh ARM_MS of focus held
 * over the drawn screen arms it again. Another screen starts it afresh.
 */

use super::moves::Shown;
use super::shows::shows_armed;
use crate::wallet::order::terms::arming::{look, Guard, Look};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Arm {
    /// The screen the guard is for; None while none shows.
    pub shown: Option<Shown>,
    pub guard: Guard,
}

/// A paint that drew `shown` whole.
pub fn painted(a: Arm, shown: Option<Shown>) -> Arm {
    let guard = if a.shown == shown { a.guard } else { Guard::default() };
    match shown {
        Some(_) => Arm { shown, guard: Guard { drawn: true, ..guard } },
        None => Arm::default(),
    }
}

/// One look at focus at `now`, `shown` on the screen.
pub fn seen(a: Arm, shown: Option<Shown>, focused: bool, now: u64) -> Arm {
    if shown.is_none() || a.shown != shown {
        return Arm { shown, guard: Guard::default() };
    }
    let guard = match look(a.guard, focused, now) {
        Look::Held(g) => g,
        Look::Lost => Guard { since: None, ..a.guard },
    };
    Arm { guard, ..a }
}

/// Whether a press that moves value on `shown` acts at `now`, after one
/// more look at focus: as the paint draws it after that look (shows.rs).
pub fn pressed(a: Arm, shown: Option<Shown>, focused: bool, now: u64) -> bool {
    shows_armed(&seen(a, shown, focused, now), shown, now)
}
