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

//! PORTSC and USBSTS bits that act when written back, and how a USB 3
//! port's link state decides its reset.

use super::super::model::{controller, op_base, CAP_LEN};
use crate::constants::{
    PORTSC_CCS, PORTSC_CHANGE_BITS, PORTSC_PED, PORTSC_PLS_SHIFT, PORTSC_PP, PORTSC_PR, PORTSC_WPR,
    USBSTS,
};
use crate::controller::{port_action, PortAction};
use crate::regs::op::{portsc_neutral, usbsts_clear};

const USBSTS_PCD: u32 = 1 << 4;
const USBSTS_EINT: u32 = 1 << 3;
const USBSTS_HSE: u32 = 1 << 2;

#[test]
fn the_neutral_portsc_never_writes_back_an_action_bit() {
    let all = u32::MAX;
    let n = portsc_neutral(all);
    assert_eq!(n & PORTSC_PED, 0, "a one in PED disables the port");
    assert_eq!(n & PORTSC_PR, 0);
    assert_eq!(n & PORTSC_WPR, 0);
    assert_eq!(n & PORTSC_CHANGE_BITS, 0, "a one in a change bit acknowledges it");
    assert_eq!(n & (1 << 16), 0, "LWS stays clear, so PLS is not written");
    assert_ne!(n & PORTSC_PP, 0, "power is kept");
    assert_eq!(n & (0x7 << 25), 0x7 << 25, "wake enables are kept");
}

#[test]
fn usbsts_acknowledges_only_what_was_asked() {
    let bar = controller(0);
    bar.present32(CAP_LEN + USBSTS as usize, USBSTS_PCD | USBSTS_EINT | USBSTS_HSE);
    usbsts_clear(op_base(&bar), USBSTS_HSE);
    assert_eq!(bar.wrote32(CAP_LEN + USBSTS as usize), USBSTS_HSE);
}

#[test]
fn a_usb3_port_decides_by_its_link_state() {
    let pls = |s: u32| s << PORTSC_PLS_SHIFT;
    assert_eq!(
        port_action(PORTSC_CCS | PORTSC_PED, true),
        PortAction::Reset,
        "trained link reset before addressing"
    );
    assert_eq!(port_action(PORTSC_CCS | pls(6), true), PortAction::WarmReset, "Inactive");
    assert_eq!(port_action(PORTSC_CCS | pls(10), true), PortAction::WarmReset, "Compliance");
    assert_eq!(port_action(PORTSC_CCS | pls(7), true), PortAction::Reset, "Polling");
    assert_eq!(port_action(PORTSC_CCS | PORTSC_PED, false), PortAction::Reset, "USB 2 always");
}
