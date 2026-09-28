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

//! The calls of process lifecycle and signals that can leave their caller
//! parked: a signal sent where only the family can say whether anyone
//! received it. Asked first by `dispatch`, so these are answered here
//! whatever it holds.

use nonos_libc::ForeignFrame;

use super::answer::Answer;
use crate::linux::abi::errno;
use crate::linux::abi::nr_sig as ns;
use crate::linux::call;
use crate::linux::guest::Guest;

pub fn answer(guest: &mut Guest, frame: &ForeignFrame) -> Option<Answer> {
    let (a, tid) = (frame.args(), frame.pid);
    Some(match frame.nr {
        ns::KILL => call::kill_from(guest, tid, a[0], a[1]),
        ns::TKILL => call::tgkill_from(guest, tid, 0, a[0], a[1]),
        ns::TGKILL if (a[0] as i64) <= 0 => Answer::value(errno::fail(errno::EINVAL)),
        ns::TGKILL => call::tgkill_from(guest, tid, a[0], a[1], a[2]),
        ns::RT_SIGQUEUEINFO => call::rt_sigqueueinfo(guest, tid, a[0], a[1], a[2]),
        ns::RT_TGSIGQUEUEINFO => call::rt_tgsigqueueinfo(guest, tid, a[0], a[1], a[2], a[3]),
        _ => return None,
    })
}
