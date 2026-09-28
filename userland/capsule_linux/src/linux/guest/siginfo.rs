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

//! What a signal carries: the 128-byte siginfo Linux hands a handler, a
//! sigtimedwait or a waitid. A pid is kept in kernel terms and becomes the
//! guest's own number only as the bytes are written, so no guest sees a
//! kernel pid in one.

pub const SI_USER: i32 = 0;
pub const INFO_LEN: usize = 128;

#[derive(Clone, Copy, Default)]
pub struct SigInfo {
    pub signo: u8,
    pub code: i32,
    /// The sending process, or the child that ended; zero names none.
    pub pid: u32,
    /// si_timerid and si_overrun, for a timer's signal, which has no pid.
    pub timer: Option<(i32, i32)>,
    /// si_value for a queued or timer signal; si_status for SIGCHLD.
    pub value: u64,
}

impl SigInfo {
    /// A signal sent by a process, as kill, tkill and sigqueue send one.
    pub fn from(signo: u8, code: i32, pid: u32) -> Self {
        SigInfo { signo, code, pid, ..SigInfo::default() }
    }

    /// Linux's layout: signo, errno, code, then at +16 either the pid and
    /// uid or the timer id and overrun, then the value or status at +24.
    pub fn bytes(&self) -> [u8; INFO_LEN] {
        let mut b = [0u8; INFO_LEN];
        b[0..4].copy_from_slice(&i32::from(self.signo).to_le_bytes());
        b[8..12].copy_from_slice(&self.code.to_le_bytes());
        let (at16, at20) = match self.timer {
            Some((id, overrun)) => (id, overrun),
            None => (guest_number(self.pid), 0), /* uid 0: every guest's */
        };
        b[16..20].copy_from_slice(&at16.to_le_bytes());
        b[20..24].copy_from_slice(&at20.to_le_bytes());
        b[24..32].copy_from_slice(&self.value.to_le_bytes());
        b
    }
}

fn guest_number(pid: u32) -> i32 {
    match pid {
        0 => 0,
        k => crate::linux::serve::guest_pid(k) as i32,
    }
}
