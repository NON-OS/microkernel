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

//! The register waits CAP.TO bounds (up to 127.5 s) give the core away
//! between polls: a millisecond's idle after every read that says wait, and
//! none once the controller is there. The admin completion wait idles before
//! each look at its clock.

use core::cell::Cell;

use nonos_libc::idles;

use crate::admin::{await_ready_before_disable, await_shutdown, wait_ready};
use crate::constants::{CSTS_RDY, CSTS_SHST_COMPLETE};
use crate::error::NvmeError;
use crate::fake::{info, up};

type Wait<'a> = &'a dyn Fn(&dyn Fn() -> u32, &dyn Fn() -> Option<u64>) -> bool;

/// Run `wait` over a CSTS that reads `csts` on a clock of 1 ms a read; give
/// whether it succeeded, the idles it made and its CSTS reads.
fn idled(csts: u32, wait: Wait) -> (bool, u64, u64) {
    let (now, reads, before) = (Cell::new(0u64), Cell::new(0u64), idles());
    let read = || {
        reads.set(reads.get() + 1);
        csts
    };
    let ok = wait(&read, &|| Some(now.replace(now.get() + 1)));
    let after = idles();
    assert_eq!(after.1 - before.1, after.0 - before.0, "each idle is one millisecond");
    (ok, after.0 - before.0, reads.get())
}

/// A controller that never gets there costs an idle after every read but
/// the last; one already there is read once and never idled for.
fn idles_until(there: u32, never_ok: bool, wait: Wait) {
    let (ok, n, reads) = idled(0, wait);
    assert!(ok == never_ok && reads > 1_000 && n == reads - 1, "{n} idles, {reads} reads");
    assert_eq!(idled(there, wait), (true, 0, 1));
}

#[test]
fn the_register_waits_idle_between_reads_and_not_once_there() {
    idles_until(CSTS_RDY, false, &|r, c| wait_ready(0, true, r, c).is_ok());
    idles_until(CSTS_RDY, true, &|r, c| await_ready_before_disable(0, r, c).is_ok());
    idles_until(CSTS_SHST_COMPLETE, false, &|r, c| await_shutdown(2_000, r, c).is_ok());
}

#[test]
fn an_unanswered_admin_command_idles_before_each_clock_read() {
    let mut up = up();
    up.fake.borrow_mut().mute = true;
    let (before, ticks) = (idles(), up.fake.borrow().ticks);
    let stride = info().doorbell_stride();
    let got = up.admin.identify_controller(up.regs, stride).err();
    assert_eq!(got, Some(NvmeError::ControllerTimeout));
    let (after, read) = (idles(), up.fake.borrow().ticks - ticks);
    // One read starts the 5 s budget; every later one follows an idle.
    assert!(read >= 50 && after.0 - before.0 == read - 1, "{read} reads, {after:?}");
}
