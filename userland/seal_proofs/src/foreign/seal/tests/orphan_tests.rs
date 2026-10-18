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

//! A supervisor that dies leaves its guests' rows orphaned, never gone: a
//! guest still on another CPU stays foreign, hosted by no one, so each call
//! it makes is refused until its own teardown removes the row.

use crate::foreign::registry::supervisor_of;
use crate::foreign::registry::{drop_rows, guest_count, guests_of, insert, is_foreign};

const ABSORB: &str = include_str!("../../../../../../src/process/foreign/seal/absorb.rs");
const ENTRY: &str = include_str!("../../../../../../src/arch/x86_64/syscall/manager/entry.rs");

#[test]
fn a_dead_supervisors_guest_stays_foreign_and_hosted_by_no_one() {
    let (sup, guest) = (9001, 9002);
    assert!(insert(guest, sup));
    drop_rows(sup);
    assert!(is_foreign(guest), "still refused as a guest");
    assert_eq!(supervisor_of(guest), None, "no supervisor to serve it");
    assert_eq!((guest_count(sup), guests_of(sup)), (0, Vec::new()));
    assert!(!insert(guest, 9003), "nobody adopts it");
    drop_rows(guest);
    assert!(!is_foreign(guest), "its own teardown removes the row");
}

#[test]
fn a_native_number_from_any_foreign_pid_is_refused_before_dispatch() {
    let gate = ABSORB.find("filter(|&p| is_foreign(p))").expect("refused() asks is_foreign");
    let host = ABSORB.find("if let Some(sup) = supervisor_of(pid)").expect("a row only if hosted");
    assert!(gate < host, "the refusal does not wait on a supervisor");
    let native = ENTRY.find("seal::native(number)").expect("entry.rs asks native()");
    let dispatch = ENTRY.find("contract_dispatch(sc,").expect("then dispatches");
    assert!(native < dispatch, "before the cap table");
}
