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

//! The in-kernel virtio-rng hands the device virtual addresses for DMA and
//! cannot deliver a byte, so it is never programmed and never available:
//! keygen's HARDWARE_ENTROPY_VERIFIED cannot rest on it.

use super::availability::{available, may_program, DELIVERS};

#[test]
fn a_driver_that_cannot_deliver_is_never_available_or_programmed() {
    assert!(!DELIVERS, "not until P9 moves it out of the kernel");
    assert!(!may_program(DELIVERS), "no bus mastering, no queue aimed at memory");
    assert!(!available(DELIVERS, true), "a device found and set up is still unavailable");
    assert!(!available(DELIVERS, false));
}

#[test]
fn the_gate_opens_only_on_delivery_and_set_up() {
    assert!(available(true, true));
    assert!(!available(true, false));
    assert!(!available(false, true));
    assert!(may_program(true));
}
