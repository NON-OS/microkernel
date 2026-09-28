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

//! A thread in sigtimedwait takes a pending signal of its set without any
//! handler running, and its call answers with that signal's number, the
//! siginfo written where it asked.

use nonos_libc::mk_foreign_reply;

use crate::linux::guest::Guest;

/// A thread in sigtimedwait takes a pending signal of its set, and its call
/// answers with that signal's number and siginfo.
pub fn taken_by_sigtimedwait(guest: &mut Guest) {
    for w in guest.signals.sigwaits.clone().into_iter().filter(|w| w.set != 0) {
        let Some(info) = guest.signals.take(w.tid, w.set) else {
            continue;
        };
        guest.signals.sigwaits.retain(|x| x.tid != w.tid);
        let value = match w.info != 0 && guest.write(w.info, &info.bytes()) < 128 {
            true => crate::linux::abi::errno::fail(crate::linux::abi::errno::EFAULT),
            false => u64::from(info.signo),
        };
        let _ = mk_foreign_reply(w.tid, value);
    }
}
