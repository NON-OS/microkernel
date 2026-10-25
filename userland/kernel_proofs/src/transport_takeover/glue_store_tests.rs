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

//! A killed caller's failed read never counts against the kept disk (L81c):
//! store_copy's compare adds a failure only for a caller still alive, so three
//! callers killed in turn cannot give the disk up for the boot.

use super::text::{between, code};

const STORE: &str = include_str!("../../../../src/hardware/block_device/store_copy/mod.rs");

#[test]
fn only_a_live_callers_failure_counts_against_the_disk() {
    let count = concat!(
        "if crate::hardware::block_device::read_store(lba, chunk).is_err() {",
        " let n = u8::from(!super::wait::caller_gone());",
        " if FAILED_CHECKS.fetch_add(n, Ordering::AcqRel) + n >= MAX_FAILED_CHECKS {",
        " STATE.store(GAVE_UP, Ordering::Release);",
    );
    let got = between(STORE, "if crate::hardware::block_device::read_store(", "crate::sys::serial");
    assert_eq!(got, code(count), "a failure counted for a caller already gone");
}
