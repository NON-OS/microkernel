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

use crate::arch::aarch64::context::save_user_frame;
use crate::arch::aarch64::exceptions::frame::ExceptionFrame;
use crate::arch::aarch64::gic::{
    acknowledge_interrupt, complete_interrupt, dispatch_irq, enter_service, leave_service,
};

use super::fatal::fatal_interrupt;
use crate::arch::aarch64::timer::is_preemption_tick;

#[no_mangle]
pub(crate) extern "C" fn aarch64_exc_irq_current(frame: *mut ExceptionFrame) {
    let frame = unsafe { &*frame };
    handle(frame, b"IRQ EL1", false)
}

#[no_mangle]
pub(crate) extern "C" fn aarch64_exc_irq_lower(frame: *mut ExceptionFrame) {
    let frame = unsafe { &*frame };

    save_user_frame(frame);
    handle(frame, b"IRQ EL0", true)
}

fn handle(frame: &ExceptionFrame, tag: &[u8], from_user: bool) {
    let Some(acked) = acknowledge_interrupt() else {
        return;
    };
    let outer = enter_service(acked);
    let handled = dispatch_irq(acked.intid);
    // A handler may have finished it already; this finishes it otherwise.
    complete_interrupt();
    leave_service(outer);
    if !handled {
        fatal_interrupt(tag, Some(acked.intid), frame)
    }
    // After the interrupt is finished, as x86_64's tick does it: a teardown
    // queued by an exit is finished here, or never on this architecture.
    if is_preemption_tick(acked.intid) {
        crate::process::exit::reclaim_at_tick(from_user);
    }
}
