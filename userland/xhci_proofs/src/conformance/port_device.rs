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

//! The device on port 1.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::FakeBar;

use super::model::{op_base, PORT1};
use crate::constants::{
    PORTSC_CCS, PORTSC_PED, PORTSC_PP, PORTSC_PR, PORTSC_PRC, PORTSC_SPEED_SHIFT,
};
use crate::controller::{park::IrqWait, reset_port};
use crate::error::XhciResult;

/// Port 1 of the controller at `bar` brought up, polled with no interrupt.
pub fn reset_port1(bar: &FakeBar, usb3: bool) -> XhciResult<u32> {
    reset_port(op_base(bar), 1, usb3, &mut IrqWait::new(0))
}

/*
 * A downstream device on port 1. PORTSC is one register both sides write, so
 * the model acts only on a value it did not present itself: that value is the
 * driver's write, and it is consumed before anything overwrites it. Connects
 * once the port is powered, finishes a reset by enabling the port and raising
 * the change bit, and drops that bit when the driver writes one to it.
 */
pub fn attached(finishes_reset: bool) -> impl Fn(&FakeBar) {
    attached_at(finishes_reset, 0)
}

/// The same device, its port showing protocol speed id `speed`.
pub fn attached_at(finishes_reset: bool, speed: u32) -> impl Fn(&FakeBar) {
    let shown = Arc::new(AtomicU32::new(0));
    move |bar| {
        let written = bar.wrote32(PORT1);
        let mut state = shown.load(Ordering::Relaxed);
        if written == state {
            return;
        }
        if written & PORTSC_PP != 0 {
            state |= PORTSC_PP | PORTSC_CCS | (speed & 0xF) << PORTSC_SPEED_SHIFT;
        }
        if written & PORTSC_PR != 0 && finishes_reset {
            state |= PORTSC_PED | PORTSC_PRC;
        }
        if written & PORTSC_PRC != 0 && written & PORTSC_PR == 0 && state & PORTSC_PED != 0 {
            state &= !PORTSC_PRC;
        }
        shown.store(state, Ordering::Relaxed);
        bar.present32(PORT1, state);
    }
}
