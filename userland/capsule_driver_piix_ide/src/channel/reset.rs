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

//! The channel's software reset, with its interrupt held down for good: the
//! driver polls, and the legacy IRQ 14/15 a compatibility-mode channel
//! raises is bound by no one.

use super::Channel;
use crate::constants::ata::{CTL_NIEN, CTL_SRST, ST_BSY, ST_FLOATING};
use crate::constants::timing::{RESET_HOLD_MS, RESET_READY_MS, RESET_RELEASE_MS};
use crate::wait;

impl Channel {
    /// Reset both devices and wait for them; false for a channel nothing
    /// drives (its status floats high) or whose devices stay busy.
    pub fn reset(&self) -> bool {
        if self.alt_status() == ST_FLOATING {
            return false;
        }
        self.control(CTL_NIEN | CTL_SRST);
        wait::hold(RESET_HOLD_MS);
        self.control(CTL_NIEN);
        wait::hold(RESET_RELEASE_MS);
        wait::until(RESET_READY_MS, || {
            let s = self.alt_status();
            s != ST_FLOATING && s & ST_BSY == 0
        })
    }
}
