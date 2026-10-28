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

//! A memfd's seals: what it starts with, what F_ADD_SEALS takes, and what
//! each seal refuses afterwards.

use crate::linux::abi::errno::{EBUSY, EINVAL, EPERM};
use crate::linux::file::memfd_seal_rules::*;

#[test]
fn only_a_memfd_that_allows_sealing_takes_seals() {
    assert_eq!(first_seals(0), SEAL);
    assert_eq!(first_seals(0x1), SEAL, "MFD_CLOEXEC alone allows nothing");
    assert_eq!(first_seals(0x2), 0);
    assert_eq!(first_seals(0x8), EXEC, "MFD_NOEXEC_SEAL allows sealing");
    assert_eq!(add(SEAL, u64::from(SHRINK), false), Err(EPERM));
}

/// What foot and wlroots clients ask for on each shm pool.
#[test]
fn seals_add_up_until_seal_itself() {
    let s = add(0, u64::from(SHRINK | GROW), false).unwrap();
    assert_eq!(s, SHRINK | GROW);
    let s = add(s, u64::from(SEAL), true).unwrap();
    assert_eq!(add(s, u64::from(WRITE), false), Err(EPERM));
    assert_eq!(add(0, 0x40, false), Err(EINVAL), "an unknown seal");
    assert_eq!(add(0, 1 << 32 | u64::from(GROW), false), Ok(GROW), "the argument is an int");
}

#[test]
fn write_waits_for_the_writable_maps_to_go() {
    assert_eq!(add(0, u64::from(WRITE), true), Err(EBUSY));
    assert_eq!(add(0, u64::from(FUTURE_WRITE), true), Ok(FUTURE_WRITE));
    assert_eq!(add(WRITE, u64::from(WRITE | GROW), true), Ok(WRITE | GROW));
    assert_eq!(add(0, u64::from(WRITE), false), Ok(WRITE));
}

#[test]
fn exec_on_a_runnable_memfd_seals_writes_and_size() {
    let all = SHRINK | GROW | WRITE | FUTURE_WRITE | EXEC;
    assert_eq!(add(0, u64::from(EXEC), false), Ok(all));
    assert_eq!(add(0, u64::from(EXEC), true), Err(EBUSY));
    assert_eq!(add(EXEC, u64::from(EXEC), true), Ok(EXEC), "already sealed against exec");
}

#[test]
fn each_seal_refuses_its_own_change() {
    assert!(resize_refused(SHRINK, 100, 50));
    assert!(!resize_refused(SHRINK, 100, 200));
    assert!(resize_refused(GROW, 100, 200));
    assert!(!resize_refused(GROW, 100, 50));
    assert!(!resize_refused(SHRINK | GROW, 100, 100), "the same size changes nothing");
    assert!(writes_refused(WRITE) && writes_refused(FUTURE_WRITE));
    assert!(!writes_refused(SEAL | SHRINK | GROW | EXEC));
    assert_eq!((F_ADD_SEALS, F_GET_SEALS), (1033, 1034));
}
