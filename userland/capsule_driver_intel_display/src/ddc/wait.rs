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

//! The waits DDC/CI's timing asks for. Each is bounded by its own length
//! on the clock (at most QUIET_US), so none can hang the driver.

use super::error::DdcError;
use super::pace::Pacer;
use crate::engine::access::{Clock, Expiry, Until};

/// Wait out `p`'s quiet window; the time the next message may go.
pub fn wait_turn(c: &dyn Clock, p: &Pacer) -> Result<u64, DdcError> {
    let now = c.now_us().ok_or(DdcError::NoClock)?;
    wait(c, p.next_at(now).saturating_sub(now));
    c.now_us().ok_or(DdcError::NoClock)
}

pub fn wait(c: &dyn Clock, us: u64) {
    let until = Until::new(c, us);
    while !until.expired() {}
}
