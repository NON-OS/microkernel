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

//! Where a handler's frame goes: on the thread's stack, or at the top of its
//! alternate stack when the handler asked for SA_ONSTACK, the stack is set
//! and the thread is not already on it; and the uc_stack the frame records,
//! which Linux saves as the stack was set, flags and all.

use crate::linux::guest::sigalt::on_stack;
use crate::linux::guest::sigstate::SA_ONSTACK;

/// The top of the alternate stack when the frame goes there, and uc_stack:
/// ss_sp, ss_flags, ss_size. `alt` is the thread's, `rsp` where it stopped.
pub fn placement(alt: [u64; 3], act_flags: u64, rsp: u64) -> (Option<u64>, [u64; 3]) {
    let [sp, _, size] = alt;
    let enabled = size != 0 && !on_stack(alt, rsp);
    let alt_top = (act_flags & SA_ONSTACK != 0 && enabled).then(|| sp.saturating_add(size));
    (alt_top, alt)
}
