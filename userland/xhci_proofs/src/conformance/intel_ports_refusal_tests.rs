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
//! When the switch-over is not made, or not taken: a machine that
//! cannot switch, a controller not Intel's, a broker that refuses, and a
//! device that ignores the write. Each comes up on the ports it already
//! has; none hangs or fails bring-up.

use nonos_libc::Call;

use super::bringup_model::found;
use super::intel_ports_model::{lynx_point, pch, ANY_BOARD, USB3_PSSEN, VAIO, XUSB2PR};
use super::intel_ports_up::{said_not_taken, up};

#[test]
fn the_vaio_that_cannot_switch_and_a_controller_not_intels_are_left_alone() {
    pch(VAIO, 0x0F, 0xFF, true);
    assert_eq!(up(lynx_point()), (false, vec![]));
    pch(ANY_BOARD, 0x0F, 0xFF, true);
    assert_eq!(up(found()), (false, vec![]), "qemu-xhci, 1b36:000d");
}

#[test]
fn firmware_that_routes_nothing_is_written_nothing() {
    pch(ANY_BOARD, 0, 0, true);
    assert_eq!(up(lynx_point()), (false, vec![]));
}

#[test]
fn a_mask_of_zero_is_never_written_as_the_unmasked_form() {
    pch(ANY_BOARD, 0, 0xFF, true);
    assert_eq!(up(lynx_point()), (true, vec![Call::PciConfig(XUSB2PR, 0xFF, 0xFF)]));
}

#[test]
fn a_broker_that_refuses_both_writes_leaves_nothing_to_hand_back() {
    pch(ANY_BOARD, 0x0F, 0xFF, true);
    nonos_libc::offer_config(&[(0x2C, ANY_BOARD), (0xD4, 0xFF), (0xDC, 0x0F)], &[], true);
    let tried = vec![Call::PciConfig(USB3_PSSEN, 0x0F, 0x0F), Call::PciConfig(XUSB2PR, 0xFF, 0xFF)];
    assert_eq!(up(lynx_point()), (false, tried), "refused, and bring-up carried on");
    said_not_taken();
}

#[test]
fn a_device_that_ignores_the_write_is_still_handed_back_what_was_admitted() {
    pch(ANY_BOARD, 0x0F, 0xFF, false);
    let expect =
        vec![Call::PciConfig(USB3_PSSEN, 0x0F, 0x0F), Call::PciConfig(XUSB2PR, 0xFF, 0xFF)];
    assert_eq!(up(lynx_point()), (true, expect));
    said_not_taken();
}
