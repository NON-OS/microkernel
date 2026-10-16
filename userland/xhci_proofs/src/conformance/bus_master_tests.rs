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

//! Bus mastering goes on only once the controller is seen halted and reset,
//! and before it is given its DCBAA and rings: on at claim, a controller a
//! predecessor left running could DMA to the frames its armed rings still
//! name before the driver had halted it; off at the ERSTBA write, its fetch
//! of the event ring segment table fails (`bus_master_erst_tests`). A
//! controller refused before that point never has it on; the refusals
//! while its memory is allocated are `bus_master_assemble_tests`.

use nonos_devmodel::FakeBar;

use super::bus_master_log::{first_bus_master_on, ran_before_bus_master, released};
use super::bus_master_seen::{DCBAA_GIVEN, ERST_GIVEN, HALTED, NOT_READY, RESET, RUNNING};
use super::left_running::attempt;
use crate::constants::{HCSPARAMS1, RTSOFF};
use crate::error::XhciError;

#[test]
fn bus_mastering_goes_on_once_the_controller_is_reset_and_before_it_is_given_its_rings() {
    let (got, log) = attempt(|_| {}, true);
    assert_eq!(got, Ok(()), "brought up: RUN was set, and the NOOP answered");
    let seen = first_bus_master_on(&log).expect("bus mastering went on");
    assert_eq!(seen & RUNNING, 0, "RUN still clear: {seen:#b}");
    assert_ne!(seen & HALTED, 0, "halted: {seen:#b}");
    assert_eq!(seen & NOT_READY, 0, "ready after the reset: {seen:#b}");
    assert_ne!(seen & RESET, 0, "reset before bus mastering: {seen:#b}");
    let given = seen & (DCBAA_GIVEN | ERST_GIVEN);
    assert_eq!(given, 0, "ERSTBA and DCBAAP not yet programmed: {seen:#b}");
    assert!(!ran_before_bus_master(&log), "RUN set after the reset, before bus mastering");
}

fn no_slots(bar: &FakeBar) {
    bar.present32(HCSPARAMS1 as usize, 2 << 24);
}

/// Runtime registers past the end of the window.
fn short_window(bar: &FakeBar) {
    bar.present32(RTSOFF as usize, 0x3000);
}

fn as_left(_: &FakeBar) {}

/// How the controller is shaped, whether it halts, and why it is refused.
type Refusal = (fn(&FakeBar), bool, XhciError);

#[test]
fn a_controller_refused_before_its_reset_never_gets_bus_mastering() {
    let refusals: [Refusal; 3] = [
        (no_slots, true, XhciError::ControllerUnsupported),
        (short_window, true, XhciError::ControllerUnsupported),
        (as_left, false, XhciError::HaltTimeout),
    ];
    for (shape, halts, refused) in refusals {
        let (got, log) = attempt(shape, halts);
        assert_eq!(got, Err(refused));
        assert_eq!(first_bus_master_on(&log), None, "{refused:?}: bus mastering on");
        assert!(released(&log), "{refused:?}: the claim given back");
    }
}
