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
//! The Intel PCH switch-over: after the legacy handoff and reset,
//! SuperSpeed terminations then USB 2.0 wires, within firmware's masks,
//! before the controller runs and so before any port is scanned; handed
//! back on teardown once the controller is halted and before the claim
//! goes, whether bring-up finished or failed.

use nonos_libc::{config_value, take_log, take_said, Call, MK_PCI_CMD_BUS_MASTER};

use super::bringup_device::Once;
use super::intel_ports_model::{at, bring_up, lynx_point, pch, routing, ANY_BOARD};
use super::intel_ports_model::{USB3_PSSEN, XUSB2PR};
use crate::constants::{USBCMD_RUN, USBSTS_HCH};
use crate::error::XhciError;

const SWITCH: [Call; 2] =
    [Call::PciConfig(USB3_PSSEN, 0x0F, 0x0F), Call::PciConfig(XUSB2PR, 0xFF, 0xFF)];
const RESTORE: [Call; 2] =
    [Call::PciConfig(XUSB2PR, 0xFF, 0), Call::PciConfig(USB3_PSSEN, 0x0F, 0)];

#[test]
fn the_ports_move_after_the_reset_and_before_the_controller_runs() {
    pch(ANY_BOARD, 0x0F, 0xFF, true);
    let (_hc, got) = bring_up(lynx_point(), Once::AnswersNoop(1));
    let driver = got.unwrap_or_else(|e| panic!("bring-up: {e:?}"));
    let log = take_log();
    let moved = routing(&log);
    assert_eq!(moved.iter().map(|m| m.0).collect::<Vec<_>>(), SWITCH, "SuperSpeed first");
    for (call, seen) in moved {
        assert_eq!(seen >> 48, 1, "{call:?} after take_over's reset");
        assert_eq!(seen as u32 & USBCMD_RUN, 0, "{call:?} before the controller runs");
    }
    let on = |c: &Call| matches!(c, Call::PciCommand(v) if v & MK_PCI_CMD_BUS_MASTER != 0);
    let on = log.iter().position(|l| on(&l.call)).expect("bus mastering on");
    assert!(on < at(&log, SWITCH[0]), "after bus mastering went on");
    assert_eq!((config_value(USB3_PSSEN), config_value(XUSB2PR)), (Some(0x0F), Some(0xFF)));
    assert!(driver.ports.is_some());
    let said: Vec<String> = take_said().into_iter().filter(|l| l.contains("port switch")).collect();
    assert_eq!(said, ["[driver_xhci] intel port switch: usb3 0x000f usb2 0x00ff\n"]);
    nonos_libc::start_log(|| 0);
    drop(driver);
    let log = take_log();
    assert_eq!(routing(&log).iter().map(|m| m.0).collect::<Vec<_>>(), RESTORE);
    assert!(at(&log, RESTORE[1]) < at(&log, Call::DeviceRelease(7)), "before the release");
    assert_eq!((config_value(USB3_PSSEN), config_value(XUSB2PR)), (Some(0), Some(0)));
}

#[test]
fn a_bring_up_that_fails_after_the_switch_hands_the_ports_back_halted() {
    pch(ANY_BOARD, 0x0F, 0xFF, true);
    let (_hc, got) = bring_up(lynx_point(), Once::AnswersNoop(5));
    assert_eq!(got.err(), Some(XhciError::CommandCompletionFailed(5)));
    let log = take_log();
    let calls: Vec<Call> = routing(&log).iter().map(|m| m.0).collect();
    assert_eq!(calls, [SWITCH, RESTORE].concat());
    let restored = &log[at(&log, RESTORE[0])];
    assert_ne!((restored.seen >> 32) as u32 & USBSTS_HCH, 0, "halted before the hand-back");
    assert!(at(&log, RESTORE[1]) < at(&log, Call::DeviceRelease(7)), "before the release");
    assert_eq!((config_value(USB3_PSSEN), config_value(XUSB2PR)), (Some(0), Some(0)));
}
