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

//! A bring-up that fails after the controller was started gives the
//! controller up in one order: halted, reset, bus mastering off, the device
//! released, and only then any DMA frame it was given. Freed first, the
//! scratchpads, rings and DCBAA went back to the kernel while a running
//! controller could still write them.

use super::bringup_device::Once;
use super::teardown_attempt::{attempt, stopped_then_released};
use crate::constants::{USBCMD_RUN, USBSTS_HCH};
use crate::error::XhciError;

#[test]
fn a_noop_that_never_completes_halts_and_resets_the_controller_before_freeing_its_dma() {
    let (got, log) = attempt(Once::Conforming);
    assert_eq!(got, Err(XhciError::CommandCompletionTimeout));
    let seen = stopped_then_released(&log);
    assert_eq!(seen as u32 & USBCMD_RUN, 0, "RUN cleared");
    assert_ne!((seen >> 32) as u32 & USBSTS_HCH, 0, "halted");
    assert_eq!(seen >> 48, 2, "reset again after the one at bring-up");
}

#[test]
fn a_noop_answered_with_an_error_is_given_up_in_the_same_order() {
    let (got, log) = attempt(Once::AnswersNoop(5));
    assert_eq!(got, Err(XhciError::CommandCompletionFailed(5)));
    let seen = stopped_then_released(&log);
    assert_ne!((seen >> 32) as u32 & USBSTS_HCH, 0, "halted");
    assert_eq!(seen >> 48, 2, "reset again after the one at bring-up");
}

#[test]
fn a_controller_that_never_halts_is_not_reset_but_still_released_before_its_dma() {
    let (got, log) = attempt(Once::NeverHalts);
    assert_eq!(got, Err(XhciError::CommandCompletionTimeout));
    let seen = stopped_then_released(&log);
    assert_eq!(seen as u32 & USBCMD_RUN, 0, "RUN cleared");
    assert_eq!((seen >> 32) as u32 & USBSTS_HCH, 0, "still running");
    assert_eq!(seen >> 48, 1, "HCRST is never written to a running controller");
}
