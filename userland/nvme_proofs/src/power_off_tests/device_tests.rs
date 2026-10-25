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

//! The power-off on the fake controller: the normal shutdown, no reset after
//! (its wait, CAP.TO, no budget covers), CC kept but for SHN, and a
//! controller gone from the bus not waited on.

use nonos_libc::calls;

use crate::constants::{CC_EN, CC_SHN_MASK, CC_SHN_NORMAL, REG_CC};
use crate::error::NvmeError;
use crate::fake::{register, up, Up};

fn order() -> Vec<&'static str> {
    let keep = ["shutdown_notified", "disabled"];
    calls().into_iter().map(|c| c.0).filter(|c| keep.contains(c)).collect()
}

#[test]
fn a_controller_is_told_waited_on_and_not_reset() {
    let Up { fake, regs, mut device, .. } = up();
    fake.borrow_mut().shut_after = 3;
    assert_eq!(device.power_off(0), Ok(()));
    assert_eq!(order().last(), Some(&"shutdown_notified"), "a reset after the shutdown");
    assert_eq!(register(regs, REG_CC) & CC_EN, CC_EN, "EN cleared, a wait no budget covers");
}

#[test]
fn a_shutdown_that_never_completes_is_given_up_at_its_budget() {
    let Up { fake, mut device, .. } = up();
    fake.borrow_mut().never_shut = true;
    let ticks = fake.borrow().ticks;
    // RTD3E 1 s: the fake's clock reads are 100 ms each.
    assert_eq!(device.power_off(1_000_000), Err(NvmeError::ShutdownTimeout));
    assert!(fake.borrow().ticks - ticks <= 10 + 2, "past the shutdown's budget");
    assert_eq!(order().last(), Some(&"shutdown_notified"));
}

#[test]
fn the_notification_keeps_cc_and_sets_shn_alone() {
    let Up { fake, regs, mut device, .. } = up();
    fake.borrow_mut().never_shut = true;
    let before = register(regs, REG_CC);
    let _ = crate::admin::shut_down(regs, 200_000);
    let after = register(regs, REG_CC);
    assert_eq!(after & 0xC000, 0x4000, "CC.SHN 01b, bits 15:14");
    assert_eq!(after & CC_SHN_MASK, CC_SHN_NORMAL);
    assert_eq!(after & !CC_SHN_MASK, before & !CC_SHN_MASK, "EN and the rest kept");
    let _ = device.disable();
}

#[test]
fn a_controller_gone_from_the_bus_is_not_waited_on() {
    let Up { fake, mut device, .. } = up();
    fake.borrow_mut().gone = true;
    assert!(device.power_off(0).is_err());
}
