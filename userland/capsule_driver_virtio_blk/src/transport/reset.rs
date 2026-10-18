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

//! Stopping the device: a reset takes its queue down, a request in flight
//! included, so nothing posted is read or written after it (plan/50 C2).

use nonos_virtio::common::reset_until;

use super::types::Transport;
use crate::clock::expired_after;
use crate::constants::{LEG_QUEUE_PFN, LEG_QUEUE_SEL, LEG_STATUS, RESET_MS};

impl Transport {
    /// Write zero to the device status and wait, by the clock, for it to
    /// read back zero; a legacy queue's page number is cleared after it, so
    /// no ring address outlives the reset. False when it never reads zero.
    pub fn reset(self) -> bool {
        match self {
            Self::Legacy(regs) => {
                regs.w8(LEG_STATUS, 0);
                let mut expired = expired_after(RESET_MS);
                let stopped = loop {
                    if regs.r8(LEG_STATUS) == 0 {
                        break true;
                    }
                    if expired() {
                        break false;
                    }
                };
                regs.w16(LEG_QUEUE_SEL, 0);
                regs.w32(LEG_QUEUE_PFN, 0);
                stopped
            }
            Self::Modern(m) => reset_until(&m.common, expired_after(RESET_MS)).is_ok(),
        }
    }
}
