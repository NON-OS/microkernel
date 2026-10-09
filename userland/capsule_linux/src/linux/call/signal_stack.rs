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

//! `sigaltstack`: the calling thread's alternate signal stack, where a
//! handler asking for SA_ONSTACK runs. The call itself is in
//! `guest::sigalt`, written over guest memory so the host proofs run it.

use nonos_libc::{mk_foreign_context, ForeignRegs};

use crate::linux::guest::Guest;

const RSP: usize = 15;

pub fn sigaltstack(guest: &mut Guest, tid: u32, ss: u64, old: u64) -> u64 {
    let alt = guest.signals.thread(tid).alt;
    let mut regs: ForeignRegs = [0; 18];
    let rsp = if mk_foreign_context(tid, &mut regs) == 0 { regs[RSP] } else { 0 };
    let (now, answer) = crate::linux::guest::sigalt::sigaltstack(guest, alt, rsp, ss, old);
    guest.signals.thread(tid).alt = now;
    answer
}
