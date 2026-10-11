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

//! A thread gone into its handler: the mask it runs under, its alternate
//! stack, and an action reset by SA_RESETHAND; then said.

use super::super::deliver_say::say;
use crate::linux::guest::sigalt::entered;
use crate::linux::guest::sigstate::{bit, SigAction, SA_NODEFER, SA_RESETHAND};
use crate::linux::guest::Guest;

/// Thread `tid`, whose mask was `blocked`, now in `act`'s handler for
/// signal `signo`.
pub(super) fn in_handler(guest: &mut Guest, tid: u32, blocked: u64, act: &SigAction, signo: u8) {
    let during = blocked | act.mask | if act.flags & SA_NODEFER != 0 { 0 } else { bit(signo) };
    guest.signals.set_blocked(tid, during);
    let thread = guest.signals.thread(tid);
    thread.saved = None;
    /* SS_AUTODISARM: the stack is off until this handler's frame returns it. */
    thread.alt = entered(thread.alt);
    if act.flags & SA_RESETHAND != 0 {
        guest.signals.set(signo as usize, SigAction::default());
    }
    say(tid, signo, act.handler);
}
