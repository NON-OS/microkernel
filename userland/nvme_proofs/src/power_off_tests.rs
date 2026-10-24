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

//! The power-off (NVMe 2.0, 3.6.2): the normal shutdown, no flush ahead of
//! it, waited out within RTD3E as capped, then the reset, whichever way the
//! wait ended; the kernel's alone. A controller that never finishes costs the budget and no
//! more.

use std::cell::Cell;

use nonos_libc::calls;

use crate::admin::{
    await_shutdown, shutdown_budget_ms, shutdown_step, ControllerIdentity, ShutdownStep,
    SHUTDOWN_CAP_MS, SHUTDOWN_DEFAULT_MS,
};
use crate::constants::{CC_EN, CC_SHN_MASK, CC_SHN_NORMAL, REG_CC};
use crate::error::NvmeError;
use crate::fake::{register, up, Up};
use crate::medium_rule::allows;
use crate::protocol::{E_IO, E_TIMEDOUT, OP_FLUSH, OP_POWER_OFF};
use crate::server::power_off_rule::status;

const OCCURRING: u32 = 1 << 2;
const COMPLETE: u32 = 2 << 2;

#[test]
fn the_budget_is_rtd3e_rounded_up_capped_or_the_default() {
    assert_eq!(shutdown_budget_ms(0), SHUTDOWN_DEFAULT_MS, "no latency named");
    assert_eq!(shutdown_budget_ms(1), 1);
    assert_eq!(shutdown_budget_ms(1_500), 2, "rounded up, never cut short");
    assert_eq!(shutdown_budget_ms(8_000_000), 8_000);
    assert_eq!(shutdown_budget_ms(30_000_000), SHUTDOWN_CAP_MS, "capped");
    assert_eq!(shutdown_budget_ms(u32::MAX), SHUTDOWN_CAP_MS);
}

#[test]
fn each_csts_read_is_named() {
    assert_eq!(shutdown_step(COMPLETE | 1, 0, 10), Ok(ShutdownStep::Complete));
    assert_eq!(shutdown_step(OCCURRING | 1, 9, 10), Ok(ShutdownStep::Wait));
    assert_eq!(shutdown_step(1, 9, 10), Ok(ShutdownStep::Wait), "not yet begun");
    assert_eq!(shutdown_step(OCCURRING, 10, 10), Err(NvmeError::ShutdownTimeout));
    assert_eq!(shutdown_step(0b10, 0, 10), Err(NvmeError::ControllerFatal), "CFS");
    assert_eq!(shutdown_step(COMPLETE | 0b10, 0, 10), Ok(ShutdownStep::Complete));
    assert_eq!(shutdown_step(u32::MAX, 0, 10), Err(NvmeError::UnsupportedController));
}

#[test]
fn a_controller_that_never_finishes_ends_at_its_budget() {
    // A millisecond a clock read; the guard turns a missing bound into a
    // failure instead of a hang.
    let (now, reads) = (Cell::new(0u64), Cell::new(0u32));
    let clock = || {
        now.set(now.get() + 1);
        assert!(now.get() < 50_000, "the wait outlived every budget");
        Some(now.get())
    };
    let csts = || {
        reads.set(reads.get() + 1);
        OCCURRING | 1
    };
    assert_eq!(await_shutdown(SHUTDOWN_CAP_MS, csts, clock), Err(NvmeError::ShutdownTimeout));
    assert!(now.get() >= SHUTDOWN_CAP_MS && now.get() <= SHUTDOWN_CAP_MS + 2, "at the budget");
    assert!(reads.get() as u64 <= SHUTDOWN_CAP_MS + 1);
}

#[test]
fn an_unreadable_clock_bounds_nothing_and_says_so() {
    assert_eq!(await_shutdown(10, || OCCURRING, || None), Err(NvmeError::ClockFailed));
}

#[test]
fn rtd3e_is_read_from_bytes_91_to_88() {
    let mut page = [0u8; 4096];
    page[0x58..0x5c].copy_from_slice(&0x0098_9680u32.to_le_bytes());
    assert_eq!(ControllerIdentity::parse(&page).rtd3e_us, 10_000_000);
}

fn order() -> Vec<&'static str> {
    let keep = ["shutdown_notified", "disabled"];
    calls().into_iter().map(|c| c.0).filter(|c| keep.contains(c)).collect()
}

#[test]
fn a_controller_is_told_waited_on_and_then_reset() {
    let Up { fake, regs, mut device, .. } = up();
    fake.borrow_mut().shut_after = 3;
    assert_eq!(device.power_off(0), Ok(()));
    assert_eq!(order().last_chunk::<2>(), Some(&["shutdown_notified", "disabled"]));
    assert_eq!(register(regs, REG_CC) & CC_EN, 0, "reset after the shutdown");
}

#[test]
fn a_shutdown_that_never_completes_is_given_up_and_the_reset_still_runs() {
    let Up { fake, mut device, .. } = up();
    fake.borrow_mut().never_shut = true;
    let ticks = fake.borrow().ticks;
    // RTD3E 1 s: the fake's clock reads are 100 ms each.
    assert_eq!(device.power_off(1_000_000), Err(NvmeError::ShutdownTimeout));
    assert!(fake.borrow().ticks - ticks <= 10 + 60, "the budget, then the reset's own wait");
    assert_eq!(order().last(), Some(&"disabled"));
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

#[test]
fn the_reply_names_how_the_shutdown_ended() {
    assert_eq!(status(Ok(())), 0);
    assert_eq!(status(Err(NvmeError::ShutdownTimeout)), E_TIMEDOUT);
    assert_eq!(status(Err(NvmeError::ControllerFatal)), E_IO);
}

#[test]
fn only_the_kernel_may_power_the_disk_off() {
    assert!(allows(OP_POWER_OFF, 0, false));
    assert!(!allows(OP_POWER_OFF, 7, false));
    assert!(!allows(OP_POWER_OFF, 7, true), "a store writer may not shut it down");
    assert!(allows(OP_FLUSH, 7, true), "while it may still flush");
}
