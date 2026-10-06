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

//! One tick: one budget of the current phase. The write hands over to the
//! read-back when the receipt arrives; the read-back hands over to the done
//! screen when the last sector matches. Either hands over to the failed
//! screen on the first error, with the disk left exactly as far as it got.

use nonos_libc::mk_time_millis;

use super::advance::{advance, Advanced};
use super::finish::finish;
use crate::install::state::{Screen, State};

/// True when something on screen changed.
pub fn tick(state: &mut State) -> bool {
    let Some(job) = state.job.as_mut() else { return false };
    let now = mk_time_millis().max(0) as u64;
    match advance(job, now) {
        Ok(Advanced::Step) => {}
        Ok(Advanced::Verifying) => state.screen = Screen::Verifying,
        Ok(Advanced::Done) => finish(state, None),
        Err(why) => finish(state, Some(why)),
    }
    true
}
