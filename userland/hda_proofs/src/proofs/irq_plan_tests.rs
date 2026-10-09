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

//! Which interrupt a controller is asked for first. A laptop's firmware that
//! expects MSI leaves the legacy line at 0xFF; such a controller must still
//! be driven, so the line is asked for only when it was routed.

#[path = "../../../capsule_driver_hda/src/setup/irq_plan.rs"]
mod irq_plan;

use irq_plan::{intx_routed, NO_LINE};

#[test]
fn a_routed_line_is_asked_for_first() {
    assert!(intx_routed(1, 11));
    assert!(intx_routed(4, 0));
}

#[test]
fn a_line_firmware_left_unrouted_is_not_asked_for() {
    assert!(!intx_routed(1, NO_LINE));
}

#[test]
fn a_function_with_no_pin_is_not_asked_for_intx() {
    assert!(!intx_routed(0, 11));
    assert!(!intx_routed(0, NO_LINE));
}
