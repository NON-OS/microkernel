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

//! The bus's state after each transfer, and the reset a run of faults ends
//! in (plan/50 C1). A fault is a transfer that found the master still busy
//! or ran out of time: a device holding the clock or the data line, or the
//! master's own state machine wedged. A NACK is the device answering.
//!
//! The reset is the bring-up again (init/bring_up.rs): on an LPSS function
//! it begins with the function reset, so the core comes back exactly as at
//! boot; a platform controller is disabled and reprogrammed. A device that
//! holds SDA low is beyond both: freeing it takes clock pulses the core
//! sends only when built with its bus-clear feature, which it does not
//! report. The bus is then named Stuck, every call still ends at its
//! deadline, and the reset is tried again after further faults.

use nonos_libc::mk_debug;

use crate::constants::{IC_STATUS, IC_STATUS_MST_ACTIVITY};
use crate::driver::Driver;
use crate::init::bring_up;
use crate::transaction::{BusState, TransferError, TransferResult, RECOVER_AFTER};

/// Count `result` into the bus's state, resetting the controller when the
/// faults in a row reach RECOVER_AFTER.
pub fn after(driver: &Driver, result: &Result<TransferResult, TransferError>) {
    let was = driver.bus.get();
    let faulted = matches!(result, Err(TransferError::Busy | TransferError::Timeout));
    let now = match (was, faulted) {
        (_, false) => BusState::Ready,
        (BusState::Ready, true) => BusState::Faulting(1),
        (BusState::Faulting(n), true) => BusState::Faulting(n.saturating_add(1)),
        (BusState::Reset, true) => BusState::Stuck(1),
        (BusState::Stuck(n), true) => BusState::Stuck(n.saturating_add(1)),
    };
    let now = match now {
        BusState::Faulting(n) if n >= RECOVER_AFTER => reset(driver, BusState::Reset),
        BusState::Stuck(n) if n >= RECOVER_AFTER => reset(driver, BusState::Stuck(0)),
        other => other,
    };
    if let Some(line) = BusState::news(was, now) {
        let _ = mk_debug(line.as_ptr(), line.len());
    }
    driver.bus.set(now);
}

/// Bring the controller up again; `freed` when it comes back idle.
fn reset(driver: &Driver, freed: BusState) -> BusState {
    let idle = bring_up(driver.regs, driver.setup).is_ok()
        && driver.regs.read32(IC_STATUS) & IC_STATUS_MST_ACTIVITY == 0;
    if idle {
        freed
    } else {
        BusState::Stuck(0)
    }
}
