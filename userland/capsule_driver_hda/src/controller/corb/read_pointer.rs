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

//! The CORB read pointer reset, a handshake some controllers only half keep.

use crate::constants::{CORBRP, CORBRP_RST};
use crate::controller::wait::until;
use crate::regs::Regs;

/// Linux waits 1000 polls of 1 us for each half of the CORBRP handshake.
const CORBRP_MS: u64 = 2;

pub(super) fn reset_read_pointer(regs: Regs) -> bool {
    unsafe { regs.w16(CORBRP, CORBRP_RST) };
    let seen = until(CORBRP_MS, || unsafe { regs.r16(CORBRP) } & CORBRP_RST != 0);
    unsafe { regs.w16(CORBRP, 0) };
    let cleared = until(CORBRP_MS, || unsafe { regs.r16(CORBRP) } & CORBRP_RST == 0);
    seen && cleared
}
