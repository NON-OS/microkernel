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

//! A signal's disposition, its number and its bit in a mask; what Linux does
//! with a signal no handler takes is in sigdefault. Dispositions are
//! process-wide, as on Linux; the numbers are transcribed.

/// The largest signal Linux defines.
pub const NSIG: usize = 64;

pub const SIGKILL: u8 = 9;
pub const SIGSEGV: u8 = 11;
pub const SIGPIPE: u8 = 13;
pub const SIGALRM: u8 = 14;
pub const SIGCHLD: u8 = 17;
pub const SIGSTOP: u8 = 19;

pub const SA_NOCLDWAIT: u64 = 2;
pub const SA_ONSTACK: u64 = 0x0800_0000;
pub const SA_RESTART: u64 = 0x1000_0000;
pub const SA_NODEFER: u64 = 0x4000_0000;
pub const SA_RESETHAND: u64 = 0x8000_0000;

/// The one bit a signal has in a mask.
pub fn bit(signum: u8) -> u64 {
    1u64 << (signum - 1)
}

/// SIGKILL and SIGSTOP are never blocked, whatever a mask asks for.
pub fn blockable(mask: u64) -> u64 {
    mask & !(bit(SIGKILL) | bit(SIGSTOP))
}

/// `struct sigaction` as the guest passes it: handler, flags, restorer, mask.
#[derive(Clone, Copy, Default)]
pub struct SigAction {
    pub handler: u64,
    pub flags: u64,
    pub restorer: u64,
    pub mask: u64,
}

impl SigAction {
    /// SIG_DFL is a null handler and SIG_IGN is 1; neither enters guest code.
    pub fn catches(&self) -> bool {
        self.handler > 1
    }
    pub fn ignores(&self) -> bool {
        self.handler == 1
    }
}
