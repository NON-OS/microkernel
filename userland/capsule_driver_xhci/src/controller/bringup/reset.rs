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
use nonos_libc::{mk_idle_ms, Deadline};

use crate::constants::USBCMD_HCRST;
use crate::error::{XhciError, XhciResult};
use crate::regs::op::{usbcmd_read, usbcmd_write};

const RESET_TIMEOUT_MS: u64 = 1_000;
/// Linux's XHCI_INTEL_HOST quirk: an Intel controller touched within about
/// a millisecond of HCRST can hang the system, so nothing reads it until
/// this has passed. Every other controller only loses the millisecond.
const POST_HCRST_MS: u64 = 1;

pub fn reset(op_base: u64) -> XhciResult<()> {
    usbcmd_write(op_base, usbcmd_read(op_base) | USBCMD_HCRST);
    let _ = mk_idle_ms(POST_HCRST_MS);
    let deadline = Deadline::after_ms(RESET_TIMEOUT_MS);
    loop {
        if usbcmd_read(op_base) & USBCMD_HCRST == 0 {
            return Ok(());
        }
        if deadline.expired() {
            return Err(XhciError::ResetTimeout);
        }
        core::hint::spin_loop();
    }
}
