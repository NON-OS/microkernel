// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! When verbs stop being answered (C1). A verb the controller fetched
//! (CORBRP on it, HDA 1.0a 3.3.19) and no codec answered is a silent codec:
//! the rings are left alone. Two in a row never fetched, and the rings are
//! reset in place (ring_reset.rs) and the verb sent once more; three resets
//! with no answer between, or rings that will not stop, and the driver uses
//! the Immediate Command interface (3.4.3) from then on. Each step is logged.

use super::immediate::exchange;
use super::verb::Link;
use crate::constants::CORBRP;
use crate::error::{HdaError, HdaResult};

/// Unanswered verbs in a row before the rings are reset.
pub const MISSED_BEFORE_RESET: u8 = 2;
/// Resets with no answer in between before the rings are given up.
pub const RESETS_BEFORE_IMMEDIATE: u8 = 3;

impl Link {
    /// Send `cmd` by whichever transport is in use, recovering a stuck ring.
    pub fn send(&mut self, cmd: u32) -> HdaResult<u32> {
        if self.health.immediate {
            return exchange(self.regs, cmd).ok_or(HdaError::VerbTimeout);
        }
        let first = self.send_ring(cmd);
        match first {
            Ok(_) => {
                self.answered();
                return first;
            }
            Err(_) if self.fetched() => return first,
            Err(_) => self.health.missed = self.health.missed.saturating_add(1),
        }
        if self.health.missed < MISSED_BEFORE_RESET {
            return first;
        }
        self.recover();
        if self.health.immediate {
            return exchange(self.regs, cmd).ok_or(HdaError::VerbTimeout);
        }
        let retry = self.send_ring(cmd);
        if retry.is_ok() {
            self.answered();
        }
        retry
    }

    /// The controller read the last verb posted: CORBRP is on it.
    fn fetched(&self) -> bool {
        // SAFETY: CORBRP lies in the register window.
        let rp = unsafe { self.regs.r16(CORBRP) };
        rp != 0xffff && rp & self.mask == self.wp
    }

    pub(super) fn answered(&mut self) {
        self.health.missed = 0;
        self.health.resets_unanswered = 0;
    }
}
