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

//! A signal as a signalfd hands it over: the 128-byte `signalfd_siginfo`,
//! laid out as Linux's, with the fields the siginfo carries moved to their
//! own places in it.

use crate::linux::guest::siginfo::{SigInfo, CLD_EXITED, CLD_KILLED};
use crate::linux::guest::sigstate::SIGCHLD;

pub const SFD_INFO_LEN: usize = 128;

impl SigInfo {
    pub fn fd_bytes(&self) -> [u8; SFD_INFO_LEN] {
        let mut b = [0u8; SFD_INFO_LEN];
        let mut put32 = |at: usize, v: u32| b[at..at + 4].copy_from_slice(&v.to_le_bytes());
        put32(0, u32::from(self.signo));
        put32(8, self.code as u32);
        let pid = match self.pid {
            0 => 0,
            k => crate::linux::serve::guest_pid(k),
        };
        let child = self.signo == SIGCHLD && matches!(self.code, CLD_EXITED | CLD_KILLED);
        match self.timer {
            /* ssi_tid is the timer's id, and ssi_overrun its overruns. */
            Some((id, overrun)) => {
                put32(24, id as u32);
                put32(32, overrun as u32);
            }
            None => put32(12, pid),
        }
        if child {
            put32(40, self.value as u32);
        } else {
            put32(44, self.value as u32);
        }
        if !child {
            b[48..56].copy_from_slice(&self.value.to_le_bytes());
        }
        b
    }
}
