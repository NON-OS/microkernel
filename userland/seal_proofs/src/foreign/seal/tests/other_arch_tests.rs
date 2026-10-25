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

//! The same gate on aarch64 and riscv64: a guest naming a native number is
//! refused with EPERM before the cap table. No guest can exist there today
//! (foreign_absent.rs), so these hold the source, not a run.

const SVC: &str = include_str!("../../../../../../src/arch/aarch64/exceptions/handlers/svc.rs");
const ECALL: &str =
    include_str!("../../../../../../src/arch/riscv64/interrupts/handlers/syscall.rs");
const ABSENT: &str = include_str!("../../../../../../src/process/foreign_absent_seal.rs");

/// `seal::native(<nr>)` comes before the dispatch and answers EPERM.
fn gated(src: &str, nr: &str) -> bool {
    let gate = format!("Some(_) if crate::process::foreign::seal::native({nr}) => {{");
    let (Some(at), Some(dispatch)) = (src.find(&gate), src.find("contract_dispatch(sc, args)"))
    else {
        return false;
    };
    let arm = &src[at..dispatch];
    at < dispatch && arm.contains("(-(errnos::EPERM as i64)) as u64") && arm.contains("Some(sc) =>")
}

#[test]
fn the_aarch64_svc_refuses_a_guests_native_number_before_dispatch() {
    assert!(gated(SVC, "frame.x8"));
}

#[test]
fn the_riscv64_ecall_refuses_a_guests_native_number_before_dispatch() {
    assert!(gated(ECALL, "frame.a7 as u64"));
}

#[test]
fn where_no_guest_exists_the_rule_still_asks_who_is_calling() {
    let body = ABSENT.split("pub(crate) fn native(").nth(1).expect("native() is there");
    let body = &body[..body.find("\n}").expect("its end")];
    assert!(body.contains("crate::process::current_pid().is_some_and(super::is_foreign)"));
    assert!(!body.contains("true"), "never refuses a native caller");
}
