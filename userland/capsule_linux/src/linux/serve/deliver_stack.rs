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
//! alternate stack when the handler asked for SA_ONSTACK and the thread is not
//! already on it; and the uc_stack the frame records, with ss_flags as
//! sigaltstack would report them at the moment the signal came.

use crate::linux::guest::sigstate::SA_ONSTACK;
use crate::linux::guest::sigthread::{SS_DISABLE, SS_ONSTACK};

/// The top of the alternate stack when the frame goes there, and uc_stack:
/// ss_sp, ss_flags, ss_size. `alt` is the thread's, `rsp` where it stopped.
pub fn placement(alt: [u64; 3], act_flags: u64, rsp: u64) -> (Option<u64>, [u64; 3]) {
    let [sp, flags, size] = alt;
    let on_alt = flags & SS_DISABLE == 0 && rsp.wrapping_sub(sp) < size;
    let alt_top = (act_flags & SA_ONSTACK != 0 && flags & SS_DISABLE == 0 && !on_alt)
        .then(|| sp.saturating_add(size));
    let ss_flags = if flags & SS_DISABLE != 0 {
        SS_DISABLE
    } else if on_alt {
        SS_ONSTACK
    } else {
        0
    };
    (alt_top, [sp, ss_flags, size])
}
