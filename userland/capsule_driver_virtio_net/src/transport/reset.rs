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

//! Stopping the device: a reset takes every queue down, so nothing posted
//! is read or written after it (plan/50 C2).

use nonos_virtio::common::reset_until;

use super::types::Transport;
use crate::clock::expired_after;
use crate::constants::{LEG_STATUS, RESET_MS};

impl Transport {
    /// Write zero to the device status and wait, by the clock, for it to
    /// read back zero. False when it never does.
    pub fn reset(self) -> bool {
        match self {
            Self::Legacy(regs) => {
                regs.w8(LEG_STATUS, 0);
                let mut expired = expired_after(RESET_MS);
                loop {
                    if regs.r8(LEG_STATUS) == 0 {
                        return true;
                    }
                    if expired() {
                        return false;
                    }
                }
            }
            Self::Modern(m) => reset_until(&m.common, expired_after(RESET_MS)).is_ok(),
        }
    }
}
